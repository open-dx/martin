extern crate iron;
extern crate env_logger;
extern crate martin_lib;
#[macro_use] extern crate log;

use std::env;
use iron::prelude::Iron;

fn main() {
    env_logger::init();

    let conn_string: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let cache_size = env::var("CACHE_SIZE").ok()
        .and_then(|cache_size| cache_size.parse::<usize>().ok())
        .unwrap_or(16384);

    let chain = martin_lib::chain(conn_string, cache_size);

    let port = 3000;
    let bind_addr = format!("0.0.0.0:{}", port);
    match Iron::new(chain).http(bind_addr.as_str()) {
        Ok(_) => info!("Server has been started on {}.", bind_addr),
        Err(err) => panic!("{:?}", err),
    };
}
