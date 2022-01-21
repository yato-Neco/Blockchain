extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::fs::File;
use std::io::Write;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::{Date, DateTime, Local, Utc};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: usize,
    hash: String,
    nonce: usize,
    transactions: Transaction,  //168
    timestamp: DateTime<Local>, //DateTime<Utc>
}
#[derive(Debug, Serialize, Deserialize)]
struct Chain {
    chain: Vec<Block>,
}

trait Chains {
    fn save(&self) -> std::io::Result<()>;
}

impl Chains for Chain {
    fn save(&self) -> std::io::Result<()> {
        let mut f = File::create("Block.json")?;
        let result = match f.write_all(serde_json::to_string(&self.chain).unwrap().as_bytes()) {
            Ok(e) => Ok(e),
            Err(e) => return Err(e),
        };

        result
    }
}

trait Blocks {
    fn create_block(&self, chain: &mut Vec<Block>) -> Block;
    fn create_hash(&self, block: Block) -> String;
    fn create_phash(&self,block: Block) -> usize;
}

impl Blocks for Transaction {
    fn create_block(&self, chain: &mut Vec<Block>) -> Block {
        if chain.len() == 0 {
            chain.push(Block {
                index: 0,
                hash: String::from(""),
                nonce: 0,
                transactions: Transaction {
                    sender: String::from("x64neco"),
                    recipient: String::from("yato"),
                    amount: 0,
                },
                timestamp: Local::now(), //Utc::now(),
            });
        }

        let block = Block {
            index: chain.len(),
            hash: self.create_hash(chain[chain.len() - 1].clone()),
            nonce: self.create_phash(chain[chain.len() - 1].clone()),
            transactions: Transaction {
                sender: self.sender.clone(),
                recipient: self.recipient.clone(),
                amount: self.amount,
            },
            timestamp: Local::now(), //Utc::now(),
        };

        chain.push(block.clone());

        block
    }

    fn create_hash(&self, block: Block) -> String {
        let index: String = block.index.to_string();
        let timestamp: String = block.timestamp.to_string();
        let hash: String = block.hash.to_string();
        let transactions: String = serde_json::to_string(&block.transactions).unwrap();
        let input: String = index + &timestamp + &hash + &transactions;

        let mut sha256 = Sha256::new();
        sha256.input_str(&input);

        sha256.result_str()
    }

    fn create_phash(&self, block: Block) -> usize {

        let index: String = block.index.to_string();
        let timestamp: String = block.timestamp.to_string();
        let hash: String = block.hash.to_string();
        let transactions: String = serde_json::to_string(&block.transactions).unwrap();
        let input: String = index + &timestamp + &hash + &transactions;

        let mut count:usize = 0;

        let start = Instant::now();


        loop {
            let mut sha256: Sha256 = Sha256::new();

            let hashmoto: String = input.clone() + &count.to_string();

            sha256.input_str(&hashmoto);

            let result: String = sha256.result_str();

            if result.starts_with("0000000") == true /*&& result.ends_with("") == true*/ {
                println!("{:?}", result);
                println!("{:?}", count);
                let end = start.elapsed();
                println!("{}.{:03}s", end.as_secs(), end.subsec_nanos() / 1_000_000);
                break count

            }

            //println!("{:?}",result);
            count = count + 1;
        }
    }
}

fn main() {
    let mut chain: Vec<Block> = Vec::new();

    let block1 = Transaction {
        sender: String::from("x64neco"),
        recipient: String::from("yato"),
        amount: 1200,
    };
    let block2 = Transaction {
        sender: String::from("a"),
        recipient: String::from("s"),
        amount: 12000,
    };
    let block3 = Transaction {
        sender: String::from("a"),
        recipient: String::from("s"),
        amount: 120,
    };

    block1.create_block(&mut chain);
    block2.create_block(&mut chain);
    block3.create_block(&mut chain);

    //println!("{:?}",block2.create_block(&mut chain));

    let chain = Chain { chain: chain };

    println!("{:?}", chain);
    //println!("{:?}",chain.save().unwrap());

    chain.save().unwrap()
    //println!("{:?}",Local::now())

    //println!("{:?}",serde_json::to_string(&Transaction{sender: String::from("x64neco"),recipient: String::from("yato"),amount: 1200}).unwrap())
}
