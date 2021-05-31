use super::block::Block;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let last_block = self.blocks.last().unwrap();
        let pre_hash = last_block.hash.clone();
        let new_block = Block::new(data, pre_hash);
        self.blocks.push(new_block);
    }
    pub fn new() -> BlockChain {
        let origin_block = Block::new("all begin from it".to_string(), "lanhao".to_string());
        BlockChain {
            blocks: vec![origin_block],
        }
    }
}
