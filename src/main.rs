use critterm::config::Config;
use critterm::critters;
use critterm::rt_neat::species;

fn main() {
    let cfg = Config::init();
    let mut cs = critters::init(&cfg.critters);
    species::sort(&mut cs);
}