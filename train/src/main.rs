use opencv::{core::Vector, face, imgcodecs, objdetect, prelude::*, types};

use std::{fs, path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src_folder = path::Path::new("image");

    // 初始化人脸检测器
    let mut face_cascade =
        objdetect::CascadeClassifier::new("../haarcascade_frontalface_default.xml")?;

    // 用于存储训练数据
    let mut images = types::VectorOfMat::new();
    let mut labels = types::VectorOfi32::new();

    // 读取目录中的文件
    if src_folder.is_dir() {
        for entry in fs::read_dir(src_folder)? {
            let entry = entry?;
            let path = entry.path();

            // 如果是图像文件
            if let Some(ext) = path.extension() {
                if ext == "jpg" || ext == "png" {
                    let img =
                        imgcodecs::imread(&path.to_string_lossy(), imgcodecs::IMREAD_GRAYSCALE)?;
                    let label = entry
                        .path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap()
                        .parse()?;
                    // 这里应该添加一些人脸检测并裁剪人脸的代码
                    let mut faces = Vector::new();
                    // 检测人脸
                    face_cascade.detect_multi_scale(
                        &img,
                        &mut faces,
                        1.1,
                        10,
                        objdetect::CASCADE_SCALE_IMAGE,
                        (30, 30).into(),
                        (0, 0).into(),
                    )?;
                    // ...
                    for face in faces {
                        let face_img = Mat::roi(&img, face)?;
                        images.push(face_img);
                        labels.push(label);
                    }
                }
            }
        }
    }

    // 创建人脸识别模型
    let mut model = face::LBPHFaceRecognizer::create(1, 8, 8, 8, 123.0)?;

    // 训练模型
    model.train(&images, &labels)?;

    // 保存模型
    model.save("../face_model.xml")?;

    Ok(())
}
