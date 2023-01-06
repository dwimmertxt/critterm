use crate::config;
use crate::rt_neat::speciation;
use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Critter {
    pub species_id:     i32,
    pub fitness:        i32,
    pub network:        network::Network,
}

impl Critter {
    pub fn new(
        cfg: &config::Critters, 
        innovations: &mut speciation::Innovations) -> Critter {
        Critter { 
            species_id: i32::default(),
            fitness: i32::default(),
            network: network::initialise(&cfg.rt_neat, innovations),
        }
    }
}