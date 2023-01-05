

use critterm::config;
use critterm::critters;

fn main() {
    let cfg = config::new();
    let mut cs = critters::Critters::new(cfg);
    println!("{:?}", cs.population[0])
}