use serde_json;

#[derive(Deserialize, Debug)]
pub struct CameraConfiguration {
    #[serde(default = "default_camera")]
    pub device : String,
    pub interval: (u32,u32),
    pub resolution: (u32,u32),
    pub format : [u8;4]
}

fn default_camera() -> String {
    r#"/dev/video0"#.to_string()
}

impl<'a> From<&'a str> for CameraConfiguration{
    fn from(conf: &'a str) -> Self {
        let cc : CameraConfiguration = serde_json::from_str(conf).unwrap();
        cc
    }
}