use blockchain::bcore::blockchain::BlockChain;

fn main() {
    let chain = BlockChain::new();
    let last = chain.blocks.last().unwrap();
}
