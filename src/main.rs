extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rscam;

use iron::prelude::*;
use iron::Handler;

use rscam::Camera;
use rscam::Frame;

use std::fmt::{self, Debug};

mod camera;
use camera::AsCameraResponse;

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

struct CameraWrapper {
    camera: Camera
}

impl CameraWrapper {
    fn new() -> CameraWrapper {
        let mut camera = rscam::new("/dev/video0").unwrap();


        for i in camera.formats() {
            if let Ok(r) = i {
                println!("for {:?}", r);

                if let Ok(r) = camera.resolutions(&r.format) {
                    println!("res {:?}", r);
                } else {
                    println!("Cannot get resolutions");
                }
            }
        }

        camera.start(&rscam::Config {
            interval: (1, 30),      // 30 fps.
            resolution: (640, 480),
            format: b"YUYV",
            ..Default::default()
        }).unwrap();


        CameraWrapper { camera }
    }

    fn capture(&self) -> ::std::io::Result<Frame>{
        self.camera.capture()
    }
}

struct CameraHandler {
    camera: CameraWrapper
}

impl CameraHandler {
    fn new(camera : CameraWrapper) -> CameraHandler {
        CameraHandler{camera}
    }
}

impl Handler for CameraHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        self.camera
            .capture()
            .or(Err(IronError::new(StringError("Cannot read frame".to_string()), iron::status::TooManyRequests)))
            .and_then(|frame|
                frame.as_camera_response()
                    .to_string()
                    .or(Err(IronError::new(StringError("Cannot read frame".to_string()), iron::status::TooManyRequests)))
                    .and_then(|json_value| Ok(Response::with((iron::status::Ok, json_value)))))
    }
}

fn main() {
    let chain = Chain::new(CameraHandler::new(CameraWrapper::new()));
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

