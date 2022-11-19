#![allow(dead_code, unused_variables, unused_imports)]

pub mod data;
pub mod server;

use crate::data::Data;
use crate::server::Server;

fn main() {
    let server = Server::new("0.0.0.0:8000");
    server.run();
}
