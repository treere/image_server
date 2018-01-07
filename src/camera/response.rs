extern crate rscam;

use serde_json;
use self::rscam::Frame;
use ::std::ops::Deref;

#[derive(Debug, Serialize)]
pub struct CameraResponse<'a> {
    res: (u32, u32),
    enc: [u8; 4],
    buff: &'a [u8],
}

impl<'a> From<&'a Frame> for CameraResponse<'a> {
    fn from(frame: &'a Frame) -> Self {
        CameraResponse { res: frame.resolution, enc: frame.format, buff: frame.deref() }
    }
}

impl<'a> CameraResponse<'a> {

    pub fn to_string(&self) -> Result<String,serde_json::Error> {
        serde_json::to_string(&self)
    }
}
