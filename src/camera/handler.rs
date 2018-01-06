

use std::fmt::{self, Debug};
use iron::prelude::*;
use iron::Handler;

use super::wrapper::CameraWrapper;
use super::AsCameraResponse;

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
    camera: CameraWrapper
}

impl CameraHandler {
    pub fn new(camera : CameraWrapper) -> CameraHandler {
        CameraHandler{camera}
    }
}

impl Handler for CameraHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.camera
            .capture()
            .or(Err(IronError::new(StringError("Cannot read frame".to_string()), ::iron::status::TooManyRequests)))
            .and_then(|frame|
                frame.as_camera_response()
                    .to_string()
                    .or(Err(IronError::new(StringError("Cannot read frame".to_string()), ::iron::status::TooManyRequests)))
                    .and_then(|json_value| Ok(Response::with((::iron::status::Ok, json_value)))))
    }
}
