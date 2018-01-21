use std::fmt::{self, Debug};
use iron::prelude::*;
use iron::Handler;
use iron::headers::ContentType;

use super::wrapper::V4l2Camera;
use camera::response::CameraResponse;



#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl ::std::error::Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

pub struct CameraHandler {
    camera: V4l2Camera
}

impl CameraHandler {
    pub fn new(camera : V4l2Camera) -> CameraHandler {
        CameraHandler{camera}
    }
}

impl Handler for CameraHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.camera
            .capture()
            .or(Err(IronError::new(StringError("Cannot read frame".to_string()), ::iron::status::TooManyRequests)))
            .and_then(|frame|
                CameraResponse::from(&frame)
                    .to_string()
                    .or(Err(IronError::new(StringError("Cannot read frame".to_string()), ::iron::status::TooManyRequests)))
                    .and_then(|json_value| Ok(Response::with((ContentType::json().0,::iron::status::Ok, json_value)))))
    }
}
