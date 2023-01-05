use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Innovations {
    pub id:             Vec<network::Connection>,
}

#[derive(Default, Debug)]
pub struct Species {
    id:             i32,
    members:        Vec<i32>,
    fitness:        f64,
}