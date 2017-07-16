extern crate hyper;
extern crate tokio_core;
extern crate roadrunner;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

mod client;
mod error;
pub mod structs;

pub use client::Client;
pub use error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
