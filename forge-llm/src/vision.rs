use anyhow::Result;
use base64::Engine;

pub struct VisionInput { pub image_base64: String, pub mime_type: String }

pub fn preprocess_image(data: &[u8]) -> Result<VisionInput> {
    let mime = if data.len() >= 4 {
        match &data[..4] {
            [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
            [0x89, b'P', b'N', b'G'] => "image/png",
            [b'G', b'I', b'F', b'8'] => "image/gif",
            [b'R', b'I', b'F', b'F'] => "image/webp",
            _ => "application/octet-stream",
        }
    } else { "application/octet-stream" };
    Ok(VisionInput {
        image_base64: base64::engine::general_purpose::STANDARD.encode(data),
        mime_type: mime.into(),
    })
}
