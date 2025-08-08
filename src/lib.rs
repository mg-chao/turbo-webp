mod utils;

use std::io::Cursor;

use image_webp::{WebPDecoder, WebPEncoder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = match WebPDecoder::new(Cursor::new(data)) {
        Ok(decoder) => decoder,
        Err(e) => {
            return Err(format!(
                "[turbo_webp::decode] Failed to decode WebP image: {}",
                e
            ));
        }
    };

    let output_buffer_size = decoder
        .output_buffer_size()
        .expect("[turbo_webp::decode] Failed to get output buffer size");

    let mut output = vec![0; output_buffer_size];
    if let Err(e) = decoder.read_image(&mut output) {
        return Err(format!(
            "[turbo_webp::decode] Failed to read WebP image: {}",
            e
        ));
    }

    Ok(output)
}

#[wasm_bindgen]
pub enum ColorType {
    L8,
    La8,
    Rgb8,
    Rgba8,
}

impl From<ColorType> for image_webp::ColorType {
    fn from(color_type: ColorType) -> Self {
        match color_type {
            ColorType::L8 => image_webp::ColorType::L8,
            ColorType::La8 => image_webp::ColorType::La8,
            ColorType::Rgb8 => image_webp::ColorType::Rgb8,
            ColorType::Rgba8 => image_webp::ColorType::Rgba8,
        }
    }
}

#[wasm_bindgen]
pub fn encode(
    data: &[u8],
    width: u32,
    height: u32,
    color_type: ColorType,
) -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity(data.len() / 16);

    let encoder = WebPEncoder::new(&mut output);

    if let Err(e) = encoder.encode(data, width, height, color_type.into()) {
        return Err(format!(
            "[turbo_webp::encode] Failed to encode WebP image: {}",
            e
        ));
    }

    Ok(output)
}
