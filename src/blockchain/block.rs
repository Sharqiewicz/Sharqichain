use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug, Clone)]
struct Header {
    timestamp: u64,
    nonce: u64,
    prev_hash: String,
    hash: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    header: Header,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut block = Block {
            header: Header {
                timestamp,
                nonce: 0,
                prev_hash: previous_hash.clone(),
                hash: "".to_string(),
            },
            transactions,
        };

        block.header.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let contents = format!(
            "{}{}{}{}",
            self.header.timestamp,
            self.header.prev_hash,
            self.header.nonce,
            self.transactions.len()
        );
        let mut hasher = Sha256::new();
        hasher.update(contents);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.header.hash[..difficulty] != target {
            self.header.nonce += 1;
            self.header.hash = self.calculate_hash();
        }
    }

    pub fn get_hash(&self) -> String {
        self.header.hash.clone()
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
}
