use critterm::config::Config;
use critterm::critters;

fn main() {
    let cfg = Config::new();
    let _cs = critters::init(&cfg.critters);
}