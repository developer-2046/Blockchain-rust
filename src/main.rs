mod block;
mod blockchain;
mod transaction;

use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    // Add transactions
    blockchain.new_transaction("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.new_transaction("Bob".to_string(), "Charlie".to_string(), 25.0);

    // Mine a block
    blockchain.add_block();

    // Add another transaction
    blockchain.new_transaction("Charlie".to_string(), "Alice".to_string(), 30.0);

    // Mine another block
    blockchain.add_block();

    println!("{:#?}", blockchain);

    if blockchain.is_chain_valid() {
        println!("The blockchain is valid!");
    } else {
        println!("The blockchain is not valid.");
    }
}

