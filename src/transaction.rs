use super::*;
use std::collections::HashSet;

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}