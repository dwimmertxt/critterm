use std::collections::HashMap;
use crate::config;
use crate::critter;
use crate::rt_neat;

#[derive(Default, Debug)]
pub struct Critters {
    pub innovations:    rt_neat::species::Innovations,
    pub population:     Vec<critter::Critter>,
    pub species:        HashMap<i32, rt_neat::species::Species>,
    
}

impl Critters {
    pub fn new(
        innovations: rt_neat::species::Innovations,
        population: Vec<critter::Critter>,
        species: HashMap<i32, rt_neat::species::Species>) -> Critters {
        Critters { innovations, population, species }
    }
}

pub fn init(cfg: &config::Critters) -> Critters {
    let mut innovations = rt_neat::species::Innovations::default();
    let mut population: Vec<critter::Critter> = Vec::new();
    for _ in 0..cfg.initial_population {
        population.push(critter::init(cfg, &mut innovations))
    }
    //let species: HashMap<i32, rt_neat::speciation::Species> = HashMap::default();
    let species = HashMap::default();
    Critters{ innovations, population, species }
}


