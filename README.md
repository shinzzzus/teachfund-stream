# TeachFund Stream

Milestone-based teacher training crowdfunding where Stellar escrow unlocks grants after verified learning outcomes.



## Problem

Teacher development funds often arrive late and lack transparent proof of completion. TeachFund Stream connects donor pledges to verified learning milestones and releases Stellar payments when outcomes are recorded.

## How It Works

1. A user connects a Freighter wallet on Stellar testnet.
2. The app opens a teacher training grant record in a Soroban Rust contract.
3. Funds move into the contract using the Stellar XLM SAC token interface.
4. A verifier records the project-specific score and status.
5. The contract releases payment to the approved recipient and leaves an auditable event trail.

## How It Uses Stellar

- Stellar testnet for fast, low-cost payment settlement.
- Freighter wallet for user-controlled signing.
- Soroban Rust smart contract for outcome-gated grant escrow.
- XLM SAC for live testnet escrow transfers.
- Soroban RPC for simulation, submission, and finality.
- Express API exposes an x402-style payment quote route.
- Uses Freighter and XLM SAC for donor pledges, Soroban for course milestones, and an x402-style quote endpoint for paid training resources.

## Track

Track 5 Social Impact

## Tech Stack

- Framework: Next.js 15 + React 19 + TypeScript
- Backend: Express + TypeScript
- Database: Prisma + SQLite
- Smart contract: Rust + Soroban SDK
- Stellar SDK: @stellar/stellar-sdk
- Wallet: Freighter
- Network: Stellar testnet

## Rust Contract API

- `open_course(id, owner, target)`
- `fund_course(id, from, amount)`
- `verify_completion(id, score, status)`
- `release_grant(id, to, amount)`
- `get_record(id)`
- `total_locked()`

## Setup & Run

```bash
git clone https://github.com/shinzzzus/teachfund-stream.git
cd teachfund-stream
npm install
npm run db:generate
npm --workspace backend run db:migrate
npm run db:seed
npm run dev
```

Frontend: http://localhost:3000
Backend: http://localhost:4000

## Contract

```bash
cd contract
cargo test
stellar contract build
```

Deploy after building:

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/teachfund_stream_contract.wasm --source alice --network testnet -- --admin alice --asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --project_name "TeachFund Stream"
```

## Network Details

- Network: Stellar testnet
- RPC URL: https://soroban-testnet.stellar.org
- Horizon URL: https://horizon-testnet.stellar.org
- XLM SAC: CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
- USDC SAC: CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA
- Contract ID: SET_CONTRACT_ID_AFTER_DEPLOY

## Docs

- docs/Submission_Guideliness.md
- docs/stellar-fullstack-cheatsheet.md
- docs/dev_setup.md
- docs/contract.md
- docs/api.md

## Originality

This is original StellarX Philippines hackathon work. It uses open-source Stellar SDKs, Soroban Rust SDK, AI-assisted development, and disclosed ecosystem integration paths. It is not a barely modified template.

## Team

- shinzzzus - @shinzzzus

## License

MIT
