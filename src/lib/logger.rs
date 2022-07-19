use std::env;

use log::info;

pub fn init_logger() {
    env::set_var("RUST_LOG", "posterior,main");

    env_logger::init();

    info!("Initialized logger");
}
