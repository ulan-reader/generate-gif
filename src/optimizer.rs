use image::{DynamicImage, ImageReader, ImageFormat, imageops::FilterType, GenericImageView};
use rayon::prelude::*;
use std::io::Cursor;

pub struct OptimizeOptions {
    pub max_width: u32,
    pub jpeg_quality: u8,
}

pub fn optimize_images(
    paths: &[String],
    opts: &OptimizeOptions,
) -> Result<Vec<DynamicImage>, String> {
    paths
        .par_iter()
        .map(|path| {
            let img = ImageReader::open(path)
                .map_err(|e| format!("Error reading {}: {}", path, e))?
                .decode()
                .map_err(|e| e.to_string())?;

            let img = resize_if_needed(img, opts.max_width);
            let img = compress_jpeg(img, opts.jpeg_quality);

            Ok(img)
        })
        .collect()
}

fn resize_if_needed(img: DynamicImage, max_width: u32) -> DynamicImage {
    let (w, _) = img.dimensions();
    if w <= max_width {
        return img;
    }

    let scale = max_width as f32 / w as f32;
    let new_h = (img.height() as f32 * scale) as u32;

    img.resize(max_width, new_h, FilterType::Triangle)
}

// Новый способ сжатия JPEG — через JpegEncoder
fn compress_jpeg(img: DynamicImage, quality: u8) -> DynamicImage {
    let mut buf = Vec::new();
    // Используем JpegEncoder напрямую
    let _encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::Jpeg)
        .expect("Failed to encode JPEG");

    // Декодируем обратно в DynamicImage
    image::load_from_memory(&buf).expect("Failed to load compressed image")
}