use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub critters: Critters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Critters {
    pub nodes:              Nodes,
    pub connection_chance:  f64,
    pub initial_population: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    pub input:  i32,
    pub hidden: i32,
    pub output: i32,
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