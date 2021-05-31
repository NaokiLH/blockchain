use blockchain::bcore::blockchain::BlockChain;

fn main() {
    let mut chain = BlockChain::new();
    chain.add_block("zhanghaitao consume 17 doller".to_string());
    chain.add_block("nibinqi consume 2003 doller".to_string());
    chain.add_block("huage consume 100000 doller".to_string());
    for i in chain.blocks {
        println!("{:#?}", i);
    }
}
