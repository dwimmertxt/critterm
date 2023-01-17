use rand::Rng;
use rand::seq::SliceRandom;
use crate::critters;
use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Innovations {
    pub id:             Vec<network::Connection>,
}

#[derive(Default, Debug)]
pub struct Species {
    id:             i32,
    members:        Vec<usize>,
    fitness:        f64,
}

pub fn sort(critters: &mut critters::Critters) {
    let mut rng = rand::thread_rng();
    let mut critter_ids = (0..critters.population.len()).collect::<Vec<usize>>();
    let mut species_id = 0;

    let mut to_remove: Vec<i32> = Vec::new();

    critters.species.entry(species_id).or_insert_with(Species::default);
    critters.species.get_mut(&species_id).unwrap().id = species_id;
    //critters.species[&species_id].id = species_id;
    let id = critter_ids.choose(&mut rng).unwrap();
    //println!("{:?}", indexes);
    critter_ids.swap_remove(*id);
    //println!("{:?}", indexes);
    //println!("{:?}", index);
    critters.species.get_mut(&species_id).unwrap().members.push(*id);
    //.push(*id);
    //critters.population[*id].species_id = species_id;

}