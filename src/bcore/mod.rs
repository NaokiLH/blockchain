pub mod block;
pub mod blockchain;
#[cfg(test)]
mod tests {
    use super::block::Block;
    #[test]
    fn work() {
        let block = Block::new("lanhao consume 30 doller".to_string(), "abc".to_string());
        println!("the hash :{}", block.header.pre_hash);
    }
}
