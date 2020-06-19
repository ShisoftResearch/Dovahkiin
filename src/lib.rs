#![feature(exact_size_is_empty)]
#![feature(plugin)]
#![crate_type = "lib"]

#[macro_use]
pub mod types;
pub mod expr;
pub mod integrated;
pub mod lexer;
pub mod parser;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bifrost;
extern crate bifrost_hasher;
extern crate byteorder;
extern crate log;
#[macro_use]
extern crate lazy_static;
