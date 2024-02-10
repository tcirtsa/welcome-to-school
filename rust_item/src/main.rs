use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct Account {
    account: String,
    psd: String,
}

#[post("/upload_image")]
async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    //先接收小程序发过来的照片并保存
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("D:\\Python\\t1\\pythonProject\\img\\1.person.1.jpg");
        let mut f = File::create(filepath).unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }

        //等待加上调用python人脸识别的代码。。。。。。

        //最后再把照片删掉
        //std::thread::sleep(std::time::Duration::from_millis(5000));
        //std::fs::remove_file("D:\\Python\\t1\\pythonProject\\img\\1.person.1.jpg");
    }
    Ok(HttpResponse::Ok().body("Ok"))
}

#[post["/account"]]
async fn check_account(a: web::Json<Account>) -> impl Responder {
    let (client, connection) = tokio_postgres::connect("postgresql://postgres:ppuqrwquqwe123@localhost/test", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let stmt = client.prepare("SELECT * FROM student WHERE account = $1 and psd = $2").await.unwrap();
    let rows = client.query(&stmt, &[&a.account, &a.psd]).await.unwrap();
    
    match rows.len() {
        0 => HttpResponse::Ok().body("Invalid username/password"),
        _ => HttpResponse::Ok().body("Login successful"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(upload_image).service(check_account))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
