mod model;
mod schema;
use actix_cors::Cors;
use actix_files as files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use handlebars::Handlebars;
use model::*;
use schema::student::dsl::*;
use serde::{Deserialize, Serialize};
use std::env;
use walkdir::WalkDir;

// 连接池类型别名
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct Welcome {
    name: String,
}

#[derive(Deserialize)]
pub struct Account {
    account: String,
    psd: String,
}
fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let welcome = Welcome {
        name: String::from("World"),
    };
    let body = hb.render("index", &welcome).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/{path}")]
async fn page(hb: web::Data<Handlebars<'_>>, path: web::Path<String>) -> impl Responder {
    let welcome = Welcome {
        name: String::from("World"),
    };
    let path = path.into_inner();
    let h = path.as_str();
    let name: Vec<&str> = h.split(".").collect();
    let body = hb.render(name[0], &welcome).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/insert_user")]
async fn insert_user(user_data: web::Json<Account>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user = NewStudent {
        account: user_data.account.clone(),
        psd: hash_password(&user_data.psd),
    };

    let result = web::block(move || {
        // 检查用户名是否存在
        match student
            .filter(account.eq(&user_data.account)) // 这里用你用来判断唯一性的字段
            .first::<Student>(&mut conn)
        {
            Ok(_) => Err(diesel::result::Error::AlreadyInTransaction), // 用户已存在
            Err(diesel::result::Error::NotFound) => {
                // 用户不存在，我们可以尝试插入新用户
                diesel::insert_into(student)
                    .values(&user)
                    .execute(&mut conn)
            }
            Err(e) => Err(e), // 其他错误
        }
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Account created"),
        _ => HttpResponse::BadRequest().json("Account already exists/other Error"),
    }
}

#[post("/update_psd")]
async fn update_psd(a: web::Json<Account>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        diesel::update(student.filter(account.eq(&a.account)))
            .set(psd.eq(&hash_password(&a.psd)))
            .execute(&mut conn)
    })
    .await;

    match result.unwrap() {
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => HttpResponse::Ok().json("Updated successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/delete_account")]
async fn delete_account(a: web::Json<Account>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // 执行删除操作
    // 执行删除操作
    match diesel::delete(student.filter(account.eq(&a.account))).execute(&mut conn) {
        Ok(_) => HttpResponse::Ok().json("Account successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// 查询所有账户并返回JSON
#[post("/get_all_accounts")]
async fn get_all_accounts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // 使用前面定义的函数来获取所有账户
    let data_result = student.load::<Student>(&mut conn); // 根据你的数据库结构调整

    match data_result {
        Ok(data) => HttpResponse::Ok().json(data), // 发送JSON响应
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 载入.env文件中的环境变量
    dotenv().ok();

    // 从环境变量获取数据库的连接字符串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建一个连接管理器
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    // 创建连接池
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut handlebars = Handlebars::new();

    // 遍历 templates 目录并注册所有模板文件
    for entry in WalkDir::new("templates").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "hbs" {
                    let template_name = path
                        .strip_prefix("templates")
                        .unwrap()
                        .with_extension("")
                        .to_str()
                        .unwrap()
                        .replace("\\", "/"); // Windows 路径转换为UNIX风格
                    handlebars
                        .register_template_file(&template_name, path)
                        .expect("Failed to register template file");
                }
            }
        }
    }

    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(handlebars_ref.clone())
            .app_data(web::Data::new(pool.clone()))
            .service(files::Files::new("/static", "./static"))
            .service(page)
            .service(index)
            .service(update_psd)
            .service(insert_user)
            .service(delete_account)
            .service(get_all_accounts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
