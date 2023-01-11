use crate::rt_neat::network;

#[derive(Default, Debug)]
pub struct Innovations {
    pub id:             Vec<network::Connection>,
}

#[derive(Default, Debug)]
pub struct Species {
    _id:             i32,
    _members:        Vec<i32>,
    _fitness:        f64,
}