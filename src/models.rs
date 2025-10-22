use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Todo {
    pub id:u32,
    pub title:String,
    pub completed:bool,
}

#[derive(Serialize, Deserialize)]
pub struct KeypairResponse {
    pub private_key: String,
    pub private_key_hex: String,
    pub public_key: String,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendEtherRequest {
    pub from_private_key: String,
    pub to_address: String,
    pub amount_eth: String,
    pub rpc_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SendEtherResponse {
    pub transaction_hash: String,
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignMessageRequest {
    pub private_key: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub message_hash: String,
    pub signer_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMessageRequest {
    pub signature: String,
    pub message: String,
    pub expected_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMessageResponse {
    pub is_valid: bool,
    pub recovered_address: String,
    pub message: String,
}


