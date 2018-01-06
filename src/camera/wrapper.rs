extern crate rscam;
use rscam::{Camera,Frame};

pub struct CameraWrapper {
    camera: Camera
}

impl CameraWrapper {
    pub fn new() -> CameraWrapper {
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

    pub fn capture(&self) -> ::std::io::Result<Frame>{
        self.camera.capture()
    }
}
