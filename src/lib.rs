#![feature(exact_size_is_empty)]
#![feature(plugin)]
#![plugin(bifrost_plugins)]
#![crate_type = "lib"]

#[macro_use]
pub mod types;
pub mod expr;
pub mod lexer;
pub mod parser;
pub mod integrated;

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bifrost;
extern crate bifrost_hasher;
extern crate byteorder;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;