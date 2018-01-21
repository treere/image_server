extern crate iron;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use iron::prelude::*;

mod camera;
use camera::{V4l2Camera,CameraHandler,CameraConfiguration};
use std::fs::File;
use std::io::{Write,Read};

fn main() {
    let conf = r#"{"interval":[1,30],"resolution":[640,480],"format":[89,85,89,86]}"#;

    let mut file = File::create("/tmp/foo.txt").unwrap();
    file.write_all(&conf.as_bytes()).unwrap();

    let conf = {
        let mut file = File::open("/tmp/foo.txt").unwrap();
        let mut content = String::default();
        file.read_to_string(& mut content).unwrap();

        CameraConfiguration::from(&content[..])
    };

    let cam = V4l2Camera::new(conf).unwrap();

    let chain = Chain::new(CameraHandler::new(cam));
    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
