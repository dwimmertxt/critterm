use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    test_value: u32,
}

pub fn new() -> Config {
    let path = std::path::Path::new("config.yaml");
    let file = match std::fs::File::open(path) {
        Err(why) => panic!("Could not open: {}: {why}", path.display()),
        Ok(file) => file,
    };
    match serde_yaml::from_reader(file) {
        Err(why) => panic!("Could not read values: {why}"),
        Ok(cfg) => cfg: Config,
    }
}