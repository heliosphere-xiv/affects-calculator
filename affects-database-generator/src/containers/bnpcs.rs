use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphqlContainer<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BNpcContainer {
    pub bnpc: Vec<BNpcMapEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BNpcMapEntry {
    pub bnpc_base: u32,
    pub bnpc_name: u32,
}
