use crate::config;
use crate::rt_neat::species;
use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Critter {
    pub species_id:     i32,
    pub fitness:        i32,
    pub network:        network::Network,
}

impl Critter {
    pub fn new(
        species_id: i32, fitness: i32, network: network::Network) -> Critter {
        Critter { species_id, fitness, network }
    }
}

pub fn init(
    cfg: &config::Critters, 
    innovations: &mut species::Innovations) -> Critter {
    let (species_id, fitness) = (0, 0);
    let network = network::init(&cfg.rt_neat, innovations);
    Critter { species_id, fitness, network }
}