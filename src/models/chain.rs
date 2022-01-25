use crate::models::block::{Block};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub chain: Vec<Block>, //ブロックを入れるlist
}