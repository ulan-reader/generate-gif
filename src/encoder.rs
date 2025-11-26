use gif::{Encoder, Frame, Repeat};
use image::{DynamicImage, GenericImageView};
use std::fs::File;

pub fn encode_gif(
    frames: &[DynamicImage],
    output_path: &str,
    delay_ms: u16
) -> Result<(), String> {
    if frames.is_empty() {
        return Err(("No frames to encode").into());
    }

    let (w, h) = frames[0].dimensions();
    let file = File::create(output_path).map_err(|err| err.to_string())?;
    let mut encoder = Encoder::new(file, w as u16, h as u16, &[]).map_err( |e| e.to_string())?;
    encoder.set_repeat(Repeat::Infinite).map_err( |e| e.to_string())?;

    for img in frames {
        let mut rgba = img.to_rgba8().into_raw();
        let mut frame = Frame::from_rgba_speed(w as u16, h as u16, &mut rgba, 10);

        frame.delay = delay_ms / 10;
        encoder.write_frame(&frame).map_err( |e| e.to_string())?;
    }

    Ok(())
}