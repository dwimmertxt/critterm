use crate::rt_neat;

#[derive(Default, Debug)]
pub struct Critter {
    pub species_id:     i32,
    pub fitness:        i32,
    pub network:        rt_neat::network::Graph,
}