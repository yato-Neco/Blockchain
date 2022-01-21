extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;


use std::fs::File;
use std::io::{Write};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::{Utc, Local, DateTime, Date};


#[derive(Debug, Serialize, Deserialize,Clone,)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

#[derive(Serialize, Deserialize, Debug,Clone,)]
struct Block 
{
    index: usize,
    hash: String,
    transactions: Transaction, //168
    timestamp:DateTime<Local>  //DateTime<Utc>
}
#[derive(Debug, Serialize, Deserialize,)]
struct Chain {
    chain: Vec<Block>
}


trait Chains {
    fn save(&self) -> std::io::Result<()>;
}


impl Chains for Chain {
    fn save(&self) ->  std::io::Result<()> {

        let mut f = File::create("Block.json")?;
        let result = match f.write_all(serde_json::to_string(&self.chain).unwrap().as_bytes()){
            Ok(_) => Ok(()),
            Err(e) => return Err(e),
        };

        result
    }
}


trait Blocks {
    fn create_block(&self,chain:&mut Vec<Block>) -> Block;
    fn create_hash(&self,block:Block) -> String;

}

impl Blocks for Transaction {

    fn create_block(&self,chain:&mut Vec<Block>) -> Block {

        if chain.len() == 0 {
            chain.push(Block {
                index: 0,
                hash: String::from(""),
                transactions:Transaction {
                    sender: String::from("x64neco"),
                    recipient: String::from("yato"),
                    amount: 0
                },
                timestamp:Local::now(), //Utc::now(),
            });
        }
        

        let block = Block {
            index: chain.len(),
            hash: self.create_hash(chain[chain.len() - 1].clone()),
            transactions: Transaction {
                sender: self.sender.clone(),
                recipient:  self.recipient.clone(),
                amount: self.amount,
            },
            timestamp:Local::now(), //Utc::now(),
        };

        chain.push(block.clone());
        

        block

    }
    

    fn create_hash(&self,block:Block) -> String {
        let index:String = block.index.to_string();
        let timestamp:String = block.timestamp.to_string();
        let hash:String = block.hash.to_string();
        let transactions:String = serde_json::to_string(&block.transactions).unwrap();
        
        let input: String = index + &timestamp + &hash + &transactions;

        let mut sha256 = Sha256::new();
        
        sha256.input_str(&input);

        sha256.result_str()
    }
}


fn main() {

    let mut chain:Vec<Block> = Vec::new();


    let block1 = Transaction {sender: String::from("x64neco"),recipient: String::from("yato"),amount: 1200};
    let block2 = Transaction {sender: String::from("a"),recipient: String::from("s"),amount: 120};
    let block3 = Transaction {sender: String::from("a"),recipient: String::from("s"),amount: 120};



    println!("{:?}",block1.create_block(&mut chain));
    block2.create_block(&mut chain);
    block3.create_block(&mut chain);


    //println!("{:?}",block2.create_block(&mut chain));

    let chain = Chain{chain:chain};

    println!("{:?}",chain.save());


    //println!("{:?}",Local::now())

    

    //println!("{:?}",serde_json::to_string(&Transaction{sender: String::from("x64neco"),recipient: String::from("yato"),amount: 1200}).unwrap())


}



