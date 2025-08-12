mod utils;

use std::io::Cursor;

use image_webp::{WebPDecoder, WebPEncoder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// 解码 WebP 图像，数据后 8 位包含图像的宽高信息
///
/// # Arguments
///
/// - `data` (`&[u8]`) - WebP 图像数据。
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - 解码后的图像数据。
/// ```
pub fn decode(data: &[u8]) -> Result<wasm_bindgen::Clamped<Vec<u8>>, String> {
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

    let mut output = vec![0; output_buffer_size + 8];
    if let Err(e) = decoder.read_image(&mut output[..output_buffer_size]) {
        return Err(format!(
            "[turbo_webp::decode] Failed to read WebP image: {}",
            e
        ));
    }

    let (width, height) = decoder.dimensions();
    let width_bytes = width.to_le_bytes();
    let height_bytes = height.to_le_bytes();

    output[output_buffer_size..output_buffer_size + 4].copy_from_slice(&width_bytes);
    output[output_buffer_size + 4..output_buffer_size + 8].copy_from_slice(&height_bytes);

    Ok(wasm_bindgen::Clamped(output))
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
