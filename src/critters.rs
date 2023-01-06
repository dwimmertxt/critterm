use std::collections::HashMap;
use crate::config;
use crate::critter;
use crate::rt_neat;

#[derive(Default, Debug)]
pub struct Critters {
    pub population:     Vec<critter::Critter>,
    pub species:        HashMap<i32, rt_neat::speciation::Species>,
    pub innovations:    rt_neat::speciation::Innovations,
}

impl Critters {
    pub fn new(cfg: config::Critters) -> Critters {
        let mut cs = Critters::default();
        cs.init_population(cfg);
        cs

    }

    fn init_population(&mut self, cfg: config::Critters) {
        for i in 0..cfg.initial_population {
            self.population.push(critter::Critter::default())
        }
        for c in &mut self.population {
            c.network.new(&cfg.rt_neat, &mut self.innovations)
        }
    }
}
