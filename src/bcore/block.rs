use chrono::prelude::*;

use crate::utils::coder::my_serialize;

pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        self.header.time = Utc::now().timestamp();
        let header = my_serialize(&self.header.time).unwrap();
        self.hash = get
    }
}
