
use serde_json;
use rscam::Frame;

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraResponse<'a> {
    res: (u32, u32),
    enc: [u8; 4],
    buff: &'a [u8],
}

impl<'a> CameraResponse<'a> {
    pub fn new(frame : &Frame) -> CameraResponse {
        use std::ops::Deref;
        CameraResponse { res: frame.resolution, enc: frame.format, buff: frame.deref() }
    }

    pub fn to_string(&self) -> Result<String,serde_json::Error> {
        serde_json::to_string(&self)
    }
}

pub trait AsCameraResponse {
    fn as_camera_response(&self) -> CameraResponse;
}

impl AsCameraResponse for Frame {
    fn as_camera_response(&self) -> CameraResponse {
        CameraResponse::new(self)
    }
}