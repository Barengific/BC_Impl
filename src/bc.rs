use super::*;
use std::collections::HashSet;

pub struct Bc {
    pub blocks: Vec<Block>,
}

// pub struct Bc {
//     pub blocks: Vec<Block>,
//     unspent_outputs: HashSet<Hash>,
// }

impl Bc{
    pub fn verify (&self) -> bool { 
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty){
                println!("Difficulty fail");
                return false;
            } else if i != 0 {
                //not genesis block
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mistmatch");
                    return false;
                }
            }else {
                // genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}
