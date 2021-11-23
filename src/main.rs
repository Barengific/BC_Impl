fn main() {
    println!("Hello, world!");
}

pub struct Block{
    pub index: u32,
    pub timestamp: u64,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub payload: String,
}

impl Block{
    pub fn new (index: u32, timestamp: u64, prev_block_hash: [u8; 16], payload: String) -> self{
        Block{
            index,
            timestamp,
            prev_block_hash,
            hash: [0;16],
            payload,
        }
    }
}