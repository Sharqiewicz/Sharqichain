mod blockchain;
use blockchain::block::Transaction;
use blockchain::Blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sharqichain = Blockchain::new();

    loop {
        let transactions = vec![
            Transaction {
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                amount: 50,
            },
            Transaction {
                from: "Bob".to_string(),
                to: "Charlie".to_string(),
                amount: 10,
            },
        ];

        sharqichain.add_block(transactions);

        if let Some(block) = sharqichain.get_last_block() {
            println!("Block added: {}", block.get_hash());
            println!("Block transactions:");
            for transaction in &block.get_transactions() {
                println!(
                    "From: {}, To: {}, Amount: {}",
                    transaction.from, transaction.to, transaction.amount
                );
            }
        }

        thread::sleep(Duration::from_secs(4));
    }
}
