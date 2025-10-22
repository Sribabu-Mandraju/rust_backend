# Rust Backend Actix - EVM Endpoints

This Rust backend provides EVM (Ethereum Virtual Machine) functionality including keypair generation, ether transactions, and message signing/verification.

## Features

1. **Generate EVM Keypair** - Create new Ethereum private/public key pairs
2. **Send Ether** - Send ether on testnet with balance validation
3. **Sign Message** - Sign messages with private keys
4. **Verify Message** - Verify message signatures

## Endpoints

### 1. Generate Keypair
**POST** `/evm/generate-keypair`

Generates a new Ethereum keypair.

**Response:**
```json
{
  "private_key": "0x...",
  "public_key": "0x...",
  "address": "0x..."
}
```

### 2. Send Ether
**POST** `/evm/send-ether`

Sends ether on testnet (Sepolia by default).

**Request:**
```json
{
  "from_private_key": "0x...",
  "to_address": "0x...",
  "amount_eth": "0.1",
  "rpc_url": "https://sepolia.infura.io/v3/YOUR_KEY" // optional
}
```

**Response (Success):**
```json
{
  "transaction_hash": "0x...",
  "success": true,
  "message": "Transaction sent successfully"
}
```

**Response (Error - 400):**
```json
{
  "transaction_hash": "",
  "success": false,
  "message": "Insufficient balance. Available: 0.05 ETH, Required: 0.1 ETH"
}
```

### 3. Sign Message
**POST** `/evm/sign-message`

Signs a message with a private key.

**Request:**
```json
{
  "private_key": "0x...",
  "message": "Hello World"
}
```

**Response:**
```json
{
  "signature": "0x...",
  "message_hash": "0x...",
  "signer_address": "0x..."
}
```

### 4. Verify Message
**POST** `/evm/verify-message`

Verifies a message signature.

**Request:**
```json
{
  "signature": "0x...",
  "message": "Hello World",
  "expected_address": "0x..."
}
```

**Response (Valid - 200):**
```json
{
  "is_valid": true,
  "recovered_address": "0x...",
  "message": "Signature is valid"
}
```

**Response (Invalid - 400):**
```json
{
  "is_valid": false,
  "recovered_address": "0x...",
  "message": "Signature is invalid - recovered address does not match expected address"
}
```

## Error Handling

- **400 Bad Request**: Invalid input data, insufficient balance, invalid signatures
- **500 Internal Server Error**: Server-side errors during keypair generation

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Run the server:
```bash
cargo run
```

3. Server runs on `http://127.0.0.1:8000`

## Testing with curl

### Generate Keypair
```bash
curl -X POST http://127.0.0.1:8000/evm/generate-keypair
```

### Send Ether (replace with actual values)
```bash
curl -X POST http://127.0.0.1:8000/evm/send-ether \
  -H "Content-Type: application/json" \
  -d '{
    "from_private_key": "0x...",
    "to_address": "0x...",
    "amount_eth": "0.01"
  }'
```

### Sign Message
```bash
curl -X POST http://127.0.0.1:8000/evm/sign-message \
  -H "Content-Type: application/json" \
  -d '{
    "private_key": "0x...",
    "message": "Hello World"
  }'
```

### Verify Message
```bash
curl -X POST http://127.0.0.1:8000/evm/verify-message \
  -H "Content-Type: application/json" \
  -d '{
    "signature": "0x...",
    "message": "Hello World",
    "expected_address": "0x..."
  }'
```

## Notes

- Default testnet is Sepolia
- Replace `YOUR_INFURA_KEY` with your actual Infura API key
- All amounts are in ETH (converted to Wei internally)
- Private keys should be 64 hex characters (with or without 0x prefix)
- Addresses should be valid Ethereum addresses
# rust_backend
# rust_backend
