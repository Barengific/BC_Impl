use bclib::*;

fn main () {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "A".to_owned(),
                    value: 21,
                },
                transaction::Output {
                    to_addr: "B".to_owned(),
                    value: 3,
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut bc = Bc::new();

    bc.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "C".to_owned(),
                    value: 10,
                },
            ],
        },
        Transaction {
            inputs: vec![
                bc.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "A".to_owned(),
                    value: 5,
                },
                transaction::Output {
                    to_addr: "B".to_owned(),
                    value: 2,
                },
            ],
        },
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    bc.update_with_block(block).expect("Failed to add block");
}

