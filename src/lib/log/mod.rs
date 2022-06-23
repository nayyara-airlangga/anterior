use std::env;

use log::info;

pub fn init_logger() {
    env::set_var("RUST_LOG", "anterior,main");

    env_logger::init();

    info!("Initialized logger");
}
