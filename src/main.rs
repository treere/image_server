extern crate iron;
extern crate rscam;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::Handler;

use rscam::Camera;

use std::fmt::{self, Debug};

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

#[derive(Debug, Serialize, Deserialize)]
struct CameraResponse<'a> {
    res: (u32, u32),
    enc: [u8; 4],
    buff: &'a [u8],
}

struct CameraHandler {
    camera: Camera
}

impl CameraHandler {
    fn new() -> CameraHandler {
        let mut camera = rscam::new("/dev/video0").unwrap();

        camera.start(&rscam::Config {
            interval: (1, 30),      // 30 fps.
            resolution: (640, 480),
            format: b"YUYV",
            ..Default::default()
        }).unwrap();


        CameraHandler { camera }
    }
}

impl Handler for CameraHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        if let Ok(frame) = self.camera.capture() {
            use std::ops::Deref;
            let resp = CameraResponse { res: frame.resolution, enc: frame.format, buff: frame.deref() };

            if let Ok(r) = serde_json::to_string(&resp) {
                Ok(Response::with((iron::status::Ok, r)))
            } else {
                Err(IronError::new(StringError("Cannot serialize to JSON".to_string()), iron::status::TooManyRequests))
            }
        } else {
            Err(IronError::new(StringError("Cannot read frame".to_string()), iron::status::TooManyRequests))
        }
    }
}

fn main() {
    let chain = Chain::new(CameraHandler::new());
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

