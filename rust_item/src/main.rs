mod model;
mod schema;
use actix_multipart::Multipart;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use model::*;
use opencv::core::{FileStorage, Mat};
use opencv::{core, face, imgcodecs, imgproc, objdetect, prelude::*, types};
use r2d2::{ConnectionManager, Pool};
use schema::latlong::dsl::*;
use schema::student::dsl::*;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use qrcode::QrCode;
use image::{Luma,ImageBuffer};
use std::io::Cursor;

// 连接池类型别名
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct Account {
    account: String,
    psd: String,
}
fn direction(a: (f64, f64), b: (f64, f64)) -> String {
    let line_start = (a.0, a.1);
    let line_end = (b.0, b.1);

    let dx = line_end.0 - line_start.0;
    let dy = line_end.1 - line_start.1;

    let tan = dy / dx;
    let tan22_5 = 0.01370864;
    let tan67_5 = 0.02285028;
    if tan < tan22_5 && tan > -tan22_5 && dx > 0f64 {
        String::from("东")
    } else if tan > tan22_5 && tan < tan67_5 && dx > 0f64 {
        String::from("东北")
    } else if (tan > tan67_5 && dx > 0f64) || (tan < -tan67_5 && dx < 0f64) {
        String::from("北")
    } else if tan > -tan67_5 && tan < -tan22_5 && dx < 0f64 {
        String::from("西北")
    } else if tan < tan22_5 && tan > -tan22_5 && dx < 0f64 {
        String::from("西")
    } else if tan > tan22_5 && tan < tan67_5 && dx < 0f64 {
        String::from("西南")
    } else if (tan > tan67_5 && dx < 0f64) || (tan < -tan67_5 && dx > 0f64) {
        String::from("南")
    } else {
        String::from("东南")
    }
}
fn point_near_line_segment(p: (f64, f64), a: (f64, f64), b: (f64, f64), range: f64) -> bool {
    let point = (p.0, p.1);
    let line_start = (a.0, a.1);
    let line_end = (b.0, b.1);

    let dx = line_end.0 - line_start.0;
    let dy = line_end.1 - line_start.1;

    // 线段长度的平方
    let l2 = dx.powi(2) + dy.powi(2);

    // 如果线段是一个点，则返回该点距离判断点的距离
    if l2 == 0.0 {
        return (point.0 - line_start.0).hypot(point.1 - line_start.1) < range;
    }

    // 计算投影点在线段上的比例 t
    let t = ((point.0 - line_start.0) * dx + (point.1 - line_start.1) * dy) / l2;
    let t = t.max(0.0).min(1.0); // 防止投影点超出线段范围，即限制t在0和1之间

    // 投影点坐标
    let projection = (line_start.0 + t * dx, line_start.1 + t * dy);

    // 判断点到投影点（在线段上）的距离是否小于指定的范围
    (point.0 - projection.0).hypot(point.1 - projection.1) < range
}

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}
fn check_image() -> Result<String, Box<dyn std::error::Error>> {
    let mut label = -1;
    let mut confidence = 0.0;
    // 初始化人脸检测器
    let mut face_detector =
        objdetect::CascadeClassifier::new("../haarcascade_frontalface_default.xml")?;

    // 初始化LBPH人脸识别器，并加载训练好的模型
    let mut recognizer = face::LBPHFaceRecognizer::create(1, 8, 8, 8, 123.0)?;
    let fs = FileStorage::new("../face_model.xml", 0, "")?;
    let fs_node = fs.get_first_top_level_node()?;
    opencv::prelude::AlgorithmTrait::read(&mut recognizer, &fs_node)?;

    // 读取图像文件
    let img = imgcodecs::imread("./person.jpg", imgcodecs::IMREAD_COLOR)?;

    // 转换为灰度图，人脸检测需要灰度图像
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, 6, 0)?;

    // 检测图像中的人脸
    let mut faces = types::VectorOfRect::new();
    face_detector.detect_multi_scale(
        &gray,
        &mut faces,
        1.1,
        10,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size::new(30, 30),
        core::Size::default(),
    )?;

    // 确认是否检测到人脸
    if !faces.is_empty() {
        // 检测到人脸，取第一个人脸进行识别
        let face = faces.get(0)?;
        let face_roi = Mat::roi(&gray, face)?;

        // 识别人脸
        recognizer.predict(&face_roi, &mut label, &mut confidence)?;

        // 根据识别结果输出标签或状态
        if label != -1 {
            Ok(format!("{}", label))
        } else {
            Ok(format!("Face not recognized"))
        }
    } else {
        Ok(format!("No faces found"))
    }
}

#[post("/upload_image")]
async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    //先接收小程序发过来的照片并保存
    while let Some(mut field) = payload.try_next().await? {
        //let content_disposition = field.content_disposition();
        //let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("./person.jpg");
        let mut f = File::create(filepath).unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }
        //最后再把照片删掉
        //std::thread::sleep(std::time::Duration::from_millis(5000));
        //std::fs::remove_file("D:\\Python\\t1\\pythonProject\\img\\1.person.1.jpg");
    }
    let h = check_image().unwrap();
    Ok(HttpResponse::Ok().body(h))
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
        Ok(_) => HttpResponse::Ok().body("Account created"),
        _ => HttpResponse::BadRequest().body("Account already exists/other Error"),
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
        Ok(_) => HttpResponse::Ok().body("Updated successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post["/check_account"]]
async fn check_account(a: web::Json<Account>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    // 查询数据库，检查用户名和密码
    let results: Student = student
        .filter(account.eq(&a.account))
        .first::<Student>(&mut conn)
        .expect("Error loading users");

    // 根据查询结果返回相应信息
    if verify(&a.psd, &results.psd).unwrap() {
        return HttpResponse::Ok().body("Password correct");
    } else {
        return HttpResponse::BadRequest().body("Invalid password");
    }
}

#[post("delete_account")]
async fn delete_account(a: web::Json<Account>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // 执行删除操作
    match diesel::delete(student.filter(account.eq(&a.account))).execute(&mut conn) {
        Ok(0) => HttpResponse::NotFound().body("Account not found"),
        Ok(_) => {
            let count: i64 = student
                .count()
                .get_result(&mut conn)
                .expect("Error counting items");
            let reset_sequence_sql =
                format!("ALTER SEQUENCE student_id_seq RESTART WITH {};", count - 2);

            // 执行SQL命令
            let _ = diesel::sql_query(&reset_sequence_sql)
                .execute(&mut conn)
                .expect("Error executing reset sequence SQL");
            HttpResponse::Ok().body("Account successfully deleted")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/how_to_go")]
async fn how_to_go(pool: web::Data<DbPool>, a: web::Json<Latlong>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let count = latlong
        .count()
        .get_result::<i64>(&mut *conn) // 注意： &*conn是为了解引用PooledConnection然后又借用
        .expect("Error loading user count");

    let results: Vec<Latlong> = latlong
        .order(id.asc())
        .load::<Latlong>(&mut conn)
        .expect("Error loading users");

    let mut h = 0;
    let mut i = 0;
    let mut s = String::new();
    while h < count - 1 {
        let point = (
            a.longitude.parse::<f64>().unwrap(),
            a.latitude.parse::<f64>().unwrap(),
        );
        let start = (
            results[i].longitude.parse::<f64>().unwrap(),
            results[i].latitude.parse::<f64>().unwrap(),
        );
        let end = (
            results[i + 1].longitude.parse::<f64>().unwrap(),
            results[i + 1].latitude.parse::<f64>().unwrap(),
        );
        if point_near_line_segment(point, start, end, 0.00004358f64) {
            s = direction(start, end);
            break;
        } else {
            i += 1;
            h += 1;
        }
    }
    if h != count - 1 {
        return HttpResponse::Ok().json(s);
    } else {
        return HttpResponse::BadRequest().finish();
    }
}

#[post("/generate_qr")]
async fn generate_qr(a: web::Json<Account>) -> impl Responder {
    let code=QrCode::new(&a.account).unwrap();
    let image:ImageBuffer<Luma<u8>,Vec<u8>>=code.render::<Luma<u8>>().build();
    let mut buffer=Cursor::new(vec![]);
    image::codecs::jpeg::JpegEncoder::new(&mut buffer).encode(&image,image.width(),image.height(),image::ColorType::L8).unwrap();
    HttpResponse::Ok().content_type("image/jpeg").body(buffer.into_inner())
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(upload_image)
            .service(check_account)
            .service(update_psd)
            .service(insert_user)
            .service(delete_account)
            .service(how_to_go)
            .service(generate_qr)
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
