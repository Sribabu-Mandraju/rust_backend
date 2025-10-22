use actix_web::{post, web, HttpResponse, Result};

use crate::models::{SendEtherRequest, SignMessageRequest, VerifyMessageRequest};
use crate::services::evm::{
    generate_evm_keypair, send_ether_transaction, sign_evm_message, verify_evm_message,
};

#[post("/evm/generate-keypair")]
pub async fn generate_keypair() -> Result<HttpResponse> {
    match generate_evm_keypair().await {
        Ok(keypair) => Ok(HttpResponse::Ok().json(keypair)),
        Err(e) => Ok(HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": format!("Failed to generate keypair: {}", e)}))),
    }
}

#[post("/evm/send-ether")]
pub async fn send_ether(req: web::Json<SendEtherRequest>) -> Result<HttpResponse> {
    match send_ether_transaction(req.into_inner()).await {
        Ok(response) => {
            if response.success {
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Transaction failed: {}", e),
            "success": false
        })) ),
    }
}

#[post("/evm/sign-message")]
pub async fn sign_message(req: web::Json<SignMessageRequest>) -> Result<HttpResponse> {
    match sign_evm_message(req.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": format!("Failed to sign message: {}", e)}))),
    }
}

#[post("/evm/verify-message")]
pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> Result<HttpResponse> {
    match verify_evm_message(req.into_inner()).await {
        Ok(response) => {
            if response.is_valid {
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": format!("Failed to verify message: {}", e)}))),
    }
}


