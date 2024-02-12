use actix_multipart::Multipart;
use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use opencv::core::{FileNode, FileStorage, FileStorage_Mode, Mat};
use opencv::{core, face, imgcodecs, imgproc, objdetect, prelude::*, types};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct Account {
    account: String,
    psd: String,
}

fn check_image() -> Result<String, Box<dyn std::error::Error>> {
    let mut label = -1;
    let mut confidence = 0.0;
    // 初始化人脸检测器
    let mut face_detector =
        objdetect::CascadeClassifier::new("../haarcascade_frontalface_default.xml")?;

    // 初始化LBPH人脸识别器，并加载训练好的模型
    let mut recognizer = face::LBPHFaceRecognizer::create(1, 8, 8, 8, 123.0)?;
    let fs = FileStorage::new("../face_model.xml", 0,"")?;
    let fs_node = fs.get_first_top_level_node()?;
    opencv::prelude::AlgorithmTrait::read(&mut recognizer,&fs_node)?;

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

#[post["/account"]]
async fn check_account(a: web::Json<Account>) -> impl Responder {
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:ppuqrwquqwe123@localhost/test", NoTls)
            .await
            .unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let stmt = client
        .prepare("SELECT * FROM student WHERE account = $1 and psd = $2")
        .await
        .unwrap();
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
