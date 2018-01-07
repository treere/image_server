extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;

mod camera;
use camera::V4l2Camera;
use camera::CameraHandler;


fn main() {

    let cam = V4l2Camera::new("/dev/video0").unwrap();

    let chain = Chain::new(CameraHandler::new(cam));
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
