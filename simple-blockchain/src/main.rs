//-----------------------------------
//          Blockchain
//-----------------------------------

use core::hash;

use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    previous_hash: String,
    timestamp: i64,
    hash: String,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            nonce: 11316,
            data: String::from("I am a first or genesis block"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000
            ",
            ),
            timestamp: Utc::now().timestamp(),
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
        };

        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!("The blockchain does not have atleast one block.");
                return;
            }

            Some(latest_block) => {
                if self.is_block_valid(&block, latest_block) {
                    self.blocks.push(block);
                    println!("The block has been successfully added");
                } else {
                    println!("Could not add block, invalid!");
                }
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("Block with id {} has wrong previous hash", new_block.id);
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        } else if new_block.id != latest_block.id + 1 {
            println!(
                "Block with id {} is not the next block after the latest block with id {}",
                new_block.id, latest_block.id
            );
            return false;
        } else if digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            &new_block.previous_hash,
            &new_block.data,
            new_block.timestamp,
            new_block.nonce
        )) != new_block.hash
        {
            println!("Block with id {} has invalid hash", new_block.id);
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("Chain is empty"),
            1 => println!("The chain only contains a single block"),
            _ => {
                for i in 1..chain.len() {
                    let previous_block = chain.get(i - 1).unwrap();
                    let current_block = chain.get(i).unwrap();

                    if !self.is_block_valid(current_block, previous_block) {
                        return false;
                    }
                }
            }
        }
        println!("The chain is valid!");
        true
    }

    // This function can only be used when the blockchain is running on a distributed network.
    fn chain_selector(&self, local: Vec<Block>, remote: Vec<Block>) -> Option<Vec<Block>> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        match (is_local_valid, is_remote_valid) {
            (true, true) => {
                if local.len() > remote.len() {
                    println!("The local copy is valid");
                    Some(local)
                } else {
                    println!("The remote copy is valid");
                    Some(remote)
                }
            }
            (true, false) => {
                println!("The local copy is valid as the remote chain is invalid");
                Some(local)
            }
            (false, true) => {
                println!("The remote copy is valid as local chain is invalid");
                Some(remote)
            }
            (false, false) => {
                println!("Both local and remote copies are invalid");
                None
            }
        }
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let now_timestamp = now.timestamp();

        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);

        Self {
            id,
            nonce,
            data,
            previous_hash,
            timestamp: now.timestamp(),
            hash,
        }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("Mining Block ID: {}", id);
        let mut nonce = 1;

        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let hash = digest(block_string);

            if hash.starts_with("0000") {
                println!("Mined block. Nonce: {},  hash: {}", nonce, hash);
                return (nonce, hash);
            }

            nonce += 1;
        }
    }
}

fn main() {
    // Initializing Genesis Block
    let mut blockchain = Blockchain::new();
    blockchain.starting_block();

    // Adding Blocks
    let block2 = Block::new(2, blockchain.blocks[0].hash.to_owned(), "Ayush".to_string());
    blockchain.try_add_block(block2);

    let block3 = Block::new(3, blockchain.blocks[1].hash.to_owned(), "Joshi".to_string());
    blockchain.try_add_block(block3);

    let block4 = Block::new(
        4,
        blockchain.blocks[2].hash.to_owned(),
        "I am 24 years old and am learning blockchain".to_string(),
    );
    blockchain.try_add_block(block4);

    let block5 = Block::new(
        5,
        blockchain.blocks[3].hash.to_owned(),
        "Decentralization".to_string(),
    );
    blockchain.try_add_block(block5);

    // Checking if the chain is valid
    blockchain.is_chain_valid(&blockchain.blocks);

    // Testing chain selector function with remote copy set to local
    blockchain.chain_selector(blockchain.blocks.to_owned(), blockchain.blocks.to_owned());

    // Printing the Blockchain
    println!();
    println!("########Printing Blockchain#########");
    println!("{:#?}", blockchain);
}
