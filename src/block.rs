use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.payload) 
    }
}


impl Block {
    pub fn new(index: u32, timestamp: u64, prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0;32],
            nonce,
            payload,
        }
    }
}

