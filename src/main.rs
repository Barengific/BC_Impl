use bclib::*;

fn main() {
    println!("Hello, world!");
    let mut block = Block::new(0,0, vec![0; 32], 0, "Genesis block!".to_owned(), 0x00ffffffffffffffffffffffffffffff);
    //0x0fffffffffffffffffffffffffffffff

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}






