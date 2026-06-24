use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CameraDevice { pub path: String, pub name: String, pub is_default: bool }

pub async fn detect_cameras() -> Result<Vec<CameraDevice>> {
    let mut cams = Vec::new();
    for i in 0..8 {
        let path = format!("/dev/video{i}");
        if std::path::Path::new(&path).exists() {
            cams.push(CameraDevice { path, name: format!("Camera {i}"), is_default: i == 0 });
        }
    }
    Ok(cams)
}
