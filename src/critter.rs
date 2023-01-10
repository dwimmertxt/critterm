use crate::config;
use crate::rt_neat::species;
use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Critter {
    pub id:             usize,
    pub species_id:     i32,
    pub fitness:        i32,
    pub network:        network::Network,
}

impl Critter {
    pub fn new(
        id: usize, species_id: i32, fitness: i32, network: network::Network) -> Critter {
        Critter { id, species_id, fitness, network }
    }
}

pub fn init(
    cfg: &config::Critters, 
    innovations: &mut species::Innovations, id: usize) -> Critter {
    let (species_id, fitness) = (0, 0);
    let network = network::init(&cfg.rt_neat, innovations);
    Critter { id, species_id, fitness, network }
}