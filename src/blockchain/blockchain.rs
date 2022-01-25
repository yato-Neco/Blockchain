extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha2::Sha512;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

extern crate colored;
use colored::*;

use crate::models::block::{Block,Transaction};
use crate::models::chain::{Chain};

extern crate serde;

use chrono::{Date, DateTime, Local, Utc};
use std::time::{Duration, Instant};

trait Chains {
    fn save(&self) -> std::io::Result<()>;  //チェーンをjson形式で保存
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

pub trait Blocks {
    fn create_block(&self, chain: &mut Vec<Block>) -> Block;    //ブロック生成関数
    fn create_hash(&self, block: &Block) -> String; //ブロックからハッシュを求める関数
    fn create_phash(&self, block: &Block) -> (usize, String);   //nonceとnonceを用いたハッシュをリターンする関数(マイニングと言われる所？)
}

impl Blocks for Transaction {
    fn create_block(&self, chain: &mut Vec<Block>) -> Block {
        if chain.len() == 0 {
            chain.push(Block {
                index: 0,
                hash: String::from(""),
                hash2: String::from(""),
                nonce: 0,
                transactions: Transaction {
                    sender: String::from("x64neco"),
                    recipient: String::from("yato"),
                    amount: 0,
                },
                timestamp: Utc::now(), //Local::now(),
            });
        }

        let phash_buf = self.create_phash(&chain[chain.len() - 1]);

        let block = Block {
            index: chain.len(),
            hash: self.create_hash(&chain[chain.len() - 1]),
            hash2: phash_buf.1,
            nonce: phash_buf.0,
            transactions: Transaction {
                sender: self.sender.clone(),
                recipient: self.recipient.clone(),
                amount: self.amount,
            },
            timestamp: Utc::now(), //Local::now(),
        };

        chain.push(block.clone());

        block
    }

    fn create_hash(&self, block: &Block) -> String {
        let index: String = block.index.to_string();
        let timestamp: String = block.timestamp.to_string();
        let hash: String = block.hash.to_string();
        let transactions: String = serde_json::to_string(&block.transactions).unwrap();
        let input: String = index + &timestamp + &hash + &transactions;

        let mut sha512 = Sha512::new();
        sha512.input_str(&input);

        //println!("{:?}",sha512.result_str());

        sha512.result_str()
    }

    fn create_phash(&self, block: &Block) -> (usize, String) {
        let input: String = self.create_hash(block);

        let mut count: usize = 0;

        let start = Instant::now();
        println!("{} {} {:?} {}","-".repeat(42),"SHA-512 Mining:".green(),block.timestamp.to_string(),"-".repeat(44));
        println!("{} {}","Hash:".red(),self.create_hash(block));

        loop {
            let mut sha512: Sha512 = Sha512::new();
            let hashmoto: String;

            if count == 0 {
                hashmoto = input.clone();
            } else {
                hashmoto = input.clone() + &count.to_string();
            }

            sha512.input_str(&hashmoto);

            let result: String = sha512.result_str();

            if result.starts_with("000000") == true
            /*&& result.ends_with("") == true*/
            {
                //println!("{:?}", input);
                println!("{} {:?}","result:".red() ,result);
                println!("{} {:?}","nonce:".red() ,count);
                let end = start.elapsed();
                println!("{} {}.{:03}s","time:".red() ,end.as_secs(), end.subsec_nanos() / 1_000_000);
                println!("{} {} {} {}","-".repeat(42),"End:".green(), Utc::now(),"-".repeat(57));
                println!("");
                break (count, result);
            }


            //println!("{:?}",count);
            count = count + 1;
        }
    }
}

/*
いらんくなったかも
#[derive(Serialize, Deserialize, Debug)]
//jsonファイルからRustの構造体に変換する構造体
struct InspeBlock {
    index: usize,
    hash: String,
    hash2: String,
    nonce: usize,
    transactions: Transactions,
    timestamp: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Transactions {
    sender: String,
    recipient: String,
    amount: usize,
}
*/

//ブロックを検証する関数(未完成)
fn inspection() {
    println!("----検証開始----");

    let file = fs::read_to_string("Block.json").expect("Failed to load JSON");

    //let file = File::open("Block.json").unwrap();
    //let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    //let u = serde_json::from_reader(reader).unwrap();
    
    let json: serde_json::Value =
        serde_json::from_str(&file).expect("JSON was not well-formatted");

    let jsons = json.as_array().unwrap().clone();//clone()使うあたりまだまだなって思う
    let mut chain: Vec<Block> = Vec::new();

    //let deserialized: InspeBlock = serde_json::from_str(&file).unwrap();
    //let deserialized: Tes = serde_json::from_reader(reader).unwrap();

    for blocks in jsons {

        let block:Block = serde_json::from_value(blocks).unwrap();
        chain.push(block);
    }


    for i in 0..chain.len() - 1{

        let index0: String = chain[i].index.to_string();
        let timestamp0: String = chain[i].timestamp.to_string();
        let hash0: String = chain[i].hash.to_string();
        let hash02: String = chain[i + 1].hash2.to_string();
        let nonce: usize = chain[i + 1].nonce;
        let transactions0: String = serde_json::to_string(&chain[i].transactions).unwrap();
        let input0: String = index0 + &timestamp0 + &hash0 + &transactions0;

        let nonce1: usize = chain[i + 1].nonce;

        

        let mut sha512_0 = Sha512::new();
        let mut sha512_1 = Sha512::new();


        sha512_0.input_str(&input0);
        

        let hashmoto = sha512_0.result_str() + &nonce1.to_string();
        sha512_1.input_str(&hashmoto);
        println!("{}",hash02);
        println!("{}",sha512_1.result_str());
        println!("{}",nonce);


        if hash02 == sha512_1.result_str() {
            println!("True");
        }else{
            println!("Fasle");
        }

        

        //sha512.result_str()

    }

    //println!("{:?}",chain[1])

    
}

fn test() {
    println!("------Start------");

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

    //println!("{:?}", chain);
    //println!("{:?}",chain.save().unwrap());
    println!("{:?}", chain.save());
    //println!("{:?}",Local::now())

    //println!("{:?}",serde_json::to_string(&Transaction{sender: String::from("x64neco"),recipient: String::from("yato"),amount: 1200}).unwrap())
    inspection();

    loop {}
}
