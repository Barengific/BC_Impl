use bclib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;
//0x0fffffffffffffffffffffffffffffff

    let mut block = Block::new(0,0, vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);
    
    block.mine();
    println!("Mined Genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut bc = Bc{
        blocks: vec![block],
    };

    println!("verify: {}", &bc.verify());

    for i in 1..=10{
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);
    
        block.mine();
        println!("{:?}", &block);

        last_hash = block.hash.clone();

        bc.blocks.push(block);

        println!("verify: {}", &bc.verify());
    }

    bc.blocks[3].index = 4;

    println!("verify: {}", &bc.verify());

    // block.hash = block.hash();

    // println!("{:?}", &block);

    // block.mine();

    // println!("{:?}", &block);
}






