use std::fmt::{ self, Debug, Formatter };
use super::*;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce:u64,
    pub payload: ,


}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}

impl Block {
}