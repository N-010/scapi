use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub amount: String,
    pub source: String,
    pub destination: String,
    #[serde(rename = "tickNumber")]
    pub tick_number: u32,
    pub timestamp: String,
    #[serde(rename = "inputType")]
    pub input_type: u32,
    #[serde(rename = "inputSize")]
    pub input_size: u32,
    #[serde(rename = "inputData")]
    pub input_data: String,
    pub signature: String,
    #[serde(rename = "moneyFlew")]
    pub money_flew: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Pagination {
    pub offset: Option<u32>,
    pub size: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Range {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Hits {
    pub total: u32,
    pub from: u32,
    pub size: u32,
}
