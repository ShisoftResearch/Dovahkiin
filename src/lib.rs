#![feature(exact_size_is_empty)]

#[macro_use]
pub mod types;

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bifrost;
extern crate bifrost_hasher;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;