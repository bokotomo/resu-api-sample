use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDevelopIndex {
    pub page: i32,
    pub page_size: i32,
}