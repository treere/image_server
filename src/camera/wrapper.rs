extern crate rscam;
use self::rscam::{Camera,Frame};
use std::fmt::Formatter;


use super::CameraConfiguration;

/// The Error of the camera
pub struct V4l2Error {
    pub description : String
}

impl ::std::fmt::Debug for V4l2Error {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "({:?})", self.description)
    }
}

/// Wrapper to rscam
pub struct V4l2Camera {
    camera: Camera
}

impl V4l2Camera {

    /// Create a new V4l2Camera.
    ///
    /// The camera, interval and resolution is hard-coded.
    pub fn new(conf : CameraConfiguration) -> Result<V4l2Camera,V4l2Error> {

        // Read the camera file.
        let camera = rscam::new(conf.device.as_ref());
        if camera.is_err() {
            return Err(V4l2Error{description:"Cannot open camera".to_string()});
        }
        let mut camera = camera.unwrap();

        // Start the camera.
        let can_start = camera.start(&rscam::Config {
            interval: conf.interval,
            resolution: conf.resolution,
            format :&conf.format,
            ..Default::default()
        });
        if can_start.is_err() {
            return Err(V4l2Error{description:"Cannot start camera".to_string()});
        }

        // Return the camera.
        Ok(V4l2Camera { camera })
    }

    /// Capture a frame
    pub fn capture(&self) -> Result<Frame,V4l2Error>{
        self.camera.capture().or(Err(V4l2Error{description:"Cannot capture frame".to_string()}))
    }
}