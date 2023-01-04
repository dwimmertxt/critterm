#![feature(type_ascription)]

mod config;
mod rt_neat;

fn main() {
    let _cfg = config::new();
    rt_neat::critters::tst();
}