#![feature(extern_types)]

mod libpq;
mod pq;

use crate::libpq::PGconn;

fn main() {
    let test_connection_string: &'static str = "postgres:///gitseed";
    let c: *const PGconn = pq::connect(test_connection_string).unwrap();
    let version = pq::server_version(c);
    print!("{version}");
    pq::exec(c, "SELECT datname FROM pg_database;");
}
