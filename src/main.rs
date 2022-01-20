extern crate crypto;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

use chrono::{Utc, Local, DateTime, Date};


extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}


struct Block {
    index: u64,
    hash: String,
    block: [i32; 6], // 126
    transactions: Transaction, //168
    timestamp:Date<Utc>  //DateTime<Utc>
}




pub trait Chain {
    fn crates(&self) -> String;
    fn crates_hash(&self) -> String;

}


impl Chain for Block {
    fn crates(&self) -> String {
        String::from("a")
    }

    fn crates_hash(&self) -> String {
        let index:String = self.index.to_string();
        let timestamp:String = self.timestamp.to_string();
        let hash:String = self.hash.to_string();
        let transactions:String = serde_json::to_string(&self.transactions).unwrap();
        
        let input: String = index + &timestamp + &hash + &transactions;

        let mut sha256 = Sha256::new();
        
        sha256.input_str(&input);

        sha256.result_str()
    }
}


fn main() {


    let block = Block {
        index: 0,
        hash: String::from(""),
        block:[1,2,5,6,8,6],
        transactions:Transaction {
            sender: String::from("x64neco"),
            recipient: String::from("yato"),
            amount: 1200
        },
        timestamp:Utc::today(), //Utc::now(),
    };


    println!("{:?}",block.crates_hash());

    //println!("{:?}",serde_json::to_string(&Transaction{sender: String::from("x64neco"),recipient: String::from("yato"),amount: 1200}).unwrap())


}



