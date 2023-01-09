use critterm::config::Config;
use critterm::critters;

fn main() {
    let cfg = Config::new();
    let cs = critters::init(&cfg.critters);
    //println!("{:?}", cs.innovations.id.len());
}