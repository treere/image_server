extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rscam;

use iron::prelude::*;

mod camera;
use camera::CameraWrapper;
use camera::CameraHandler;


fn main() {
    let chain = Chain::new(CameraHandler::new(CameraWrapper::new()));
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
