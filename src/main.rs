use rustblockchain::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    
    let mut genesis_block = Block::new(0, now(), vec![0;32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output{
                    to_address: "Alice".to_owned(),
                    value: 50000,
                }, 
                transaction::Output{
                    to_address: "Bob".to_owned(),
                    value: 7000,
                },
            ],
        },

    ] , difficulty);

    //block.hash = block.hash();
    
   // println!( "{:?}", &block);

    genesis_block.mine();
    println!("Mined Genesis {:?}", &genesis_block);

   // last_hash = block.hash.clone();

  let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add  block");
    

   

    //println!( "{:?}", &block);

   // let h = block.hash();
    
   // println!( "{:?}", &h);

   // block.hash = h;
   // println!( "{:?}", &block);

  // blockchain.blocks[3].hash[3] += 4;
   //println!("Verify: {}", &blockchain.verify());


}
