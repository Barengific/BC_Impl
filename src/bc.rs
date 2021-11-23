use super::*;
use std::collections::HashSet;

pub struct Bc {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}