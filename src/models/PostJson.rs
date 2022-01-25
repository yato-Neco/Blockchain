#[derive(Debug, Serialize, Deserialize, Clone)]
//取引の構造体
pub struct PostTransaction {
    pub sender: String, //送り主
    pub recipient: String, //受取
    pub amount: i64, //金
}