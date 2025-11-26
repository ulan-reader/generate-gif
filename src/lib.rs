mod optimizer;
mod encoder;

pub use optimizer::{optimize_images, OptimizeOptions};
pub use encoder::encode_gif;

use image::DynamicImage;

/// Удобная обёртка – делает всё:
/// оптимизация + ресайз + генерация GIF
pub fn make_gif(
    input_paths: &[String],
    output_path: &str,
    options: OptimizeOptions,
    delay_ms: u16,
) -> Result<(), String> {

    let images: Vec<DynamicImage> = optimize_images(input_paths, &options)?;

    encoder::encode_gif(&images, output_path, delay_ms)?;

    Ok(())
}
