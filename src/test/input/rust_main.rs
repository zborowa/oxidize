//#![deny(warnings)]

//extern crate bootstrap;

//use std::env;

//use bootstrap::{Flags, Config, Build};

fn main() {
//    let args = env::args().skip(1).collect::<Vec<_>>();
//    let flags = Flags::parse(&args);
    let mut config = Config::parse(&flags.build, flags.config.clone());

//    if std::fs::metadata("config.mk").is_ok() {
//        config.update_with_config_mk();
//    }

//    Build::new(flags, config).build();
}