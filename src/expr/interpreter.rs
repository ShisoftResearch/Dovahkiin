use std::collections::{HashMap, LinkedList};
use types::Value;

#[derive(Debug)]
pub struct Envorinment {
    bindings: HashMap<u64, LinkedList<Value>>
}