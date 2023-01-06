

use critterm::config::Config;
use critterm::critters::Critters;

fn main() {
    let cfg = Config::new();
    let cs = Critters::new(&cfg.critters);
}