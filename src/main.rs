use bclib::*;

fn main() {
    println!("Hello, world!");
    let mut block = Block::new(0,0, vec![0; 32], 0, "Genesis block".to_owned());

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
}






