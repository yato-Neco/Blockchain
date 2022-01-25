use chrono::{Date, DateTime, Local, Utc};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: usize, //ブロックのindex
    pub hash: String, //nonceなしハッシュ(前のブロック)
    pub hash2: String, //nonceありハッシュ(前のブロック)
    pub nonce: usize, //ハッシュのルールを設ける数値
    pub transactions: Transaction, //取引
    pub timestamp: DateTime<Utc>,  //DateTime<Utc>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
//取引の構造体
pub struct Transaction {
    pub sender: String, //送り主
    pub recipient: String, //受取
    pub amount: i64, //金
}