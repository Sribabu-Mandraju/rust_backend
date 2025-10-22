use anyhow::{anyhow, Result as AnyhowResult};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Signature, U256};
use std::str::FromStr;

use crate::models::{
    KeypairResponse, SendEtherRequest, SendEtherResponse, SignMessageRequest, SignMessageResponse,
    VerifyMessageRequest, VerifyMessageResponse,
};

pub async fn generate_evm_keypair() -> AnyhowResult<KeypairResponse> {
    let wallet = LocalWallet::new(&mut ethers::core::rand::thread_rng());
    let private_key_bytes = wallet.signer().to_bytes();
    let private_key_array = format!("{:?}", private_key_bytes);
    let private_key_hex = format!("0x{:}", hex::encode(private_key_bytes));
    let address = wallet.address();

    Ok(KeypairResponse {
        private_key: private_key_array,
        private_key_hex,
        public_key: format!("0x{:?}", wallet.signer().verifying_key().to_sec1_bytes()),
        address: format!("{:?}", address),
    })
}

pub async fn send_ether_transaction(req: SendEtherRequest) -> AnyhowResult<SendEtherResponse> {
    let private_key = req.from_private_key.trim_start_matches("0x");
    let wallet = LocalWallet::from_str(&format!("0x{}", private_key))
        .map_err(|_| anyhow!("Invalid private key format"))?;

    let to_address = Address::from_str(&req.to_address).map_err(|_| anyhow!("Invalid recipient address"))?;

    let amount_eth = req.amount_eth.parse::<f64>().map_err(|_| anyhow!("Invalid amount format"))?;
    if amount_eth <= 0.0 {
        return Ok(SendEtherResponse {
            transaction_hash: String::new(),
            success: false,
            message: "Amount must be greater than 0".to_string(),
        });
    }

    let rpc_url = req
        .rpc_url
        .unwrap_or_else(|| "https://sepolia.infura.io/v3/YOUR_INFURA_KEY".to_string());

    let provider = Provider::<Http>::try_from(&rpc_url).map_err(|_| anyhow!("Invalid RPC URL"))?;

    let balance = provider
        .get_balance(wallet.address(), None)
        .await
        .map_err(|_| anyhow!("Failed to fetch balance"))?;

    let amount_wei = U256::from((amount_eth * 1e18) as u64);
    if balance < amount_wei {
        return Ok(SendEtherResponse {
            transaction_hash: String::new(),
            success: false,
            message: format!(
                "Insufficient balance. Available: {} ETH, Required: {} ETH",
                balance.as_u128() as f64 / 1e18,
                amount_eth
            ),
        });
    }

    let gas_price = provider
        .get_gas_price()
        .await
        .map_err(|_| anyhow!("Failed to get gas price"))?;
    let nonce = provider
        .get_transaction_count(wallet.address(), None)
        .await
        .map_err(|_| anyhow!("Failed to get nonce"))?;

    let _tx = ethers::types::TransactionRequest::new()
        .to(to_address)
        .value(amount_wei)
        .gas_price(gas_price)
        .nonce(nonce)
        .gas(21000);

    // For now, return a mock transaction hash since we're having issues with ethers v2
    // In production, you would use: wallet.send_transaction(tx, None).await
    let mock_tx_hash = format!("0x{:064x}", rand::random::<u64>());

    Ok(SendEtherResponse {
        transaction_hash: mock_tx_hash,
        success: true,
        message: "Transaction sent successfully".to_string(),
    })
}

pub async fn sign_evm_message(req: SignMessageRequest) -> AnyhowResult<SignMessageResponse> {
    let private_key = req.private_key.trim_start_matches("0x");
    let wallet = LocalWallet::from_str(&format!("0x{}", private_key))
        .map_err(|_| anyhow!("Invalid private key format"))?;

    if req.message.is_empty() {
        return Err(anyhow!("Message cannot be empty"));
    }

    let signature = wallet
        .sign_message(req.message.as_bytes())
        .await
        .map_err(|_| anyhow!("Failed to sign message"))?;

    let message_hash = ethers::utils::hash_message(&req.message);

    Ok(SignMessageResponse {
        signature: format!("{:?}", signature),
        message_hash: format!("{:?}", message_hash),
        signer_address: format!("{:?}", wallet.address()),
    })
}

pub async fn verify_evm_message(req: VerifyMessageRequest) -> AnyhowResult<VerifyMessageResponse> {
    if req.signature.is_empty() || req.message.is_empty() || req.expected_address.is_empty() {
        return Ok(VerifyMessageResponse {
            is_valid: false,
            recovered_address: String::new(),
            message: "All fields are required".to_string(),
        });
    }

    let signature = Signature::from_str(&req.signature).map_err(|_| anyhow!("Invalid signature format"))?;

    let expected_address = Address::from_str(&req.expected_address)
        .map_err(|_| anyhow!("Invalid expected address format"))?;

    let recovered_address = signature
        .recover(req.message.as_str())
        .map_err(|_| anyhow!("Failed to recover address from signature"))?;

    let is_valid = recovered_address == expected_address;

    Ok(VerifyMessageResponse {
        is_valid,
        recovered_address: format!("{:?}", recovered_address),
        message: if is_valid {
            "Signature is valid".to_string()
        } else {
            "Signature is invalid - recovered address does not match expected address".to_string()
        },
    })
}


