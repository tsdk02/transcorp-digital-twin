# Transcorp Digital Twin

A mock server that simulates Transcorp's card issuance and transaction management APIs. Built with Actix-web in Rust, it provides a local development and testing environment that mirrors real Transcorp API behavior without requiring a live connection.

## Features

- **KYC** - OTP generation, customer registration
- **Card Management** - Add, list, block/unlock, replace, physical card requests, preferences
- **Transactions** - Create, fetch by external ID, fetch by entity
- **Corporate Registration** - Register business entities
- **Balance Inquiry** - Fetch card/entity balances
- **Auth Middleware** - Token and tenant validation
- **State Persistence** - Saves in-memory state to `data/state.json` on shutdown (Ctrl+C)
- **Webhook Forwarding** - Optional partner webhook callbacks

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/kyc/customer/generate/otp` | Generate KYC OTP |
| POST | `/kyc/v2/register` | Register customer |
| POST | `/Yappay/txn-manager/create` | Create transaction |
| GET | `/Yappay/txn-manager/fetch/{extTrxId}` | Fetch transaction by external ID |
| GET | `/Yappay/txn-manager/fetch/success/entity/{entityId}` | Fetch transactions by entity |
| POST | `/Yappay/business-entity-manager/addCard` | Add a card |
| GET | `/Yappay/business-entity-manager/fetchbalance/{entityId}` | Fetch balance |
| POST | `/Yappay/business-entity-manager/v3/getCardList` | List cards |
| POST | `/Yappay/business-entity-manager/block` | Lock/unlock/block card |
| POST | `/Yappay/business-entity-manager/replaceCard` | Replace card |
| POST | `/Yappay/business-entity-manager/requestPhysicalCard` | Request physical card |
| POST | `/Yappay/business-entity-manager/setPreferences` | Set card preferences |
| POST | `/Yappay/business-entity-manager/fetchPreference` | Fetch card preferences |
| POST | `/Yappay/registration-manager/register` | Register corporate entity |

## Prerequisites

- Rust 1.70+

## Quick Start

```bash
# Clone
git clone https://github.com/tsdk02/transcorp-digital-twin.git
cd transcorp-digital-twin

# Run
cargo run
```

The server starts on `http://localhost:8080` by default.

## Configuration

Set via environment variables or a `.env` file:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | Server port |
| `PARTNER_WEBHOOK_URL` | _(none)_ | URL for partner webhook callbacks |
| `VALID_AUTH_TOKENS` | `test-token` | Comma-separated list of valid auth tokens |
| `VALID_TENANTS` | `BUSINESS,SANDBOXTEST` | Comma-separated list of valid tenant IDs |

## Postman Collection

A Postman collection is included at `postman/transcorp-digital-twin.postman_collection.json` for quick API testing.

## Project Structure

```
src/
  main.rs          # Server bootstrap, shutdown hook
  config.rs        # Environment-based configuration
  routes.rs        # Route definitions
  state.rs         # In-memory application state
  handlers/        # Request handlers (card, kyc, registration, transaction, webhook)
  middleware/      # Auth validation middleware
  models/          # Data models (card, corporate, customer, transaction, envelope, error)
  services/        # Business logic (balance, ID generation)
```
