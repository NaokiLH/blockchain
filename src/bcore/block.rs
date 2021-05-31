use chrono::prelude::*;

use crate::utils::coder::{get_hash, my_serialize};

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
    pub fn new(data: String, pre_hash: String) -> Block {
        let time = Utc::now().timestamp();
        let transactions = my_serialize(&data).unwrap();
        let tx_hash = get_hash(&transactions[..]);
        let hash = get_hash(&my_serialize(&time).unwrap());

        let block = Block {
            header: BlockHeader {
                time: time,
                tx_hash: tx_hash,
                pre_hash: pre_hash,
            },
            hash: hash,
            data: data,
        };
        block
    }
}
