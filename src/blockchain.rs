pub mod block;
use self::block::Block;
use self::block::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(vec![], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 4,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.get_last_block().unwrap().get_hash();
        let mut new_block = Block::new(transactions, previous_hash);
        new_block.mine(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}
