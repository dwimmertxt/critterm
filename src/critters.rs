use std::collections::HashMap;
use crate::config;
use crate::critter;
use crate::rt_neat;

#[derive(Default, Debug)]
pub struct Critters {
    pub innovations:    rt_neat::speciation::Innovations,
    pub population:     Vec<critter::Critter>,
    pub species:        HashMap<i32, rt_neat::speciation::Species>,
    
}

impl Critters {
    pub fn new(cfg: &config::Critters) -> Critters {
        let mut innovations = rt_neat::speciation::Innovations::default();
        let mut population: Vec<critter::Critter> = Vec::new();
        for _ in 0..cfg.initial_population {
            population.push(
                critter::Critter::new(cfg, &mut innovations))
        }
        // species
        let species: HashMap<i32, rt_neat::speciation::Species> = HashMap::default();
        Critters{ innovations, population, species }
    }
}




