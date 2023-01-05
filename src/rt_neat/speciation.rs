use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Innovations {
    id:             Vec<network::Connection>,
    count:          i32,
}

#[derive(Default, Debug)]
pub struct Species {
    id:             i32,
    members:        Vec<i32>,
    fitness:        f64,
}