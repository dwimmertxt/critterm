use std::collections::HashMap;



pub struct Innovations {
    id:             Vec<Connection>,
    count:          i32,
}

pub struct Species {
    id:             i32,
    members:        Vec<i32>,
    fitness:        f64,
}

pub struct Critter {
    species_id:     i32,
    fitness:        i32,
    network:        Network,
}

pub struct Critters {
    population:     Vec<Critter>,
    species:        HashMap,
    innovations:    Innovations,
}