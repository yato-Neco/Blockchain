extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha2::Sha512;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use chrono::{Date, DateTime, Local, Utc};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize, Clone)]
//取引の構造体
struct Transaction {
    sender: String, //送り主
    recipient: String, //受取
    amount: i64, //金
}

//ブロックの構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: usize, //ブロックのindex
    hash: String, //nonceなしハッシュ(前のブロック)
    hash2: String, //nonceありハッシュ(前のブロック)
    nonce: usize, //ハッシュのルールを設ける数値
    transactions: Transaction, //取引
    timestamp: DateTime<Utc>,  //DateTime<Utc>
}

//チェーン部分の構造体
#[derive(Debug, Serialize, Deserialize)]
struct Chain {
    chain: Vec<Block>, //ブロックを入れるlist
}

//チェーンを部分の操作する関数？
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

trait Blocks {
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

        let block = Block {
            index: chain.len(),
            hash: self.create_hash(&chain[chain.len() - 1]),
            hash2: self.create_phash(&chain[chain.len() - 1]).1,
            nonce: self.create_phash(&chain[chain.len() - 1]).0,
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

            if result.starts_with("0000") == true
            /*&& result.ends_with("") == true*/
            {
                println!("{:?}", input);
                println!("{:?}", result);
                println!("{:?}", count);
                let end = start.elapsed();
                println!("{}.{:03}s", end.as_secs(), end.subsec_nanos() / 1_000_000);
                break (count, result);
            }

            //println!("{:?}",result);
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

    let jsons = json.as_array().unwrap().clone();
    let mut chain: Vec<Block> = Vec::new();

    //let deserialized: InspeBlock = serde_json::from_str(&file).unwrap();
    //let deserialized: Tes = serde_json::from_reader(reader).unwrap();

    for i in jsons {

        let block:Block = serde_json::from_value(i).unwrap();
        chain.push(block);
    }

    println!("{:?}",chain)

    
}

fn main() {
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
