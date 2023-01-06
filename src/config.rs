use serde::{Deserialize, Serialize};
use serde_yaml::{self};


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub critters: Critters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Critters {
    pub rt_neat:            RtNeat,
    pub initial_population: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RtNeat {
    pub nodes:      Nodes,
    pub mutation:   Mutation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes {
    pub input:              i32,
    pub hidden:             i32,
    pub output:             i32,
    pub connection_chance:  f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mutation {
    pub weight:         Weight,
    pub new_connection: f64,
    pub new_node:       f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weight {
    pub chance:         f64,
    pub adj_threshold:  f64,
    pub add_threshold:  f64,
    pub add_factor:     f64,
    pub sub_factor:     f64,
}

impl Config {
    pub fn new() -> Config {
        Config { critters: Config::parse_critters() }
    }

    fn parse_critters() -> Critters {
        let mut path = std::path::Path::new("critters.yaml");
        let file = match std::fs::File::open(path) {
            Err(why) => panic!("Could not open: {}: {why}", path.display()),
            Ok(file) => file,
        };
        match serde_yaml::from_reader(file) {
            Err(why) => panic!("Could not read values: {why}"),
            Ok(cfg) => cfg: Critters,
        }
    }
}
