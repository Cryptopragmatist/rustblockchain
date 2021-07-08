use rustblockchain::*;

fn main() {
    let block = Block::new(0, 0, vec![0;32], 0, "Genesis".to_owned());
    
    println!( "{:?}", &block);
}
