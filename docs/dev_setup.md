# Dev Setup

1. Install Node.js 20+, Rust, and Stellar CLI.
2. Fund a test identity:

```bash
stellar keys generate --global alice --network testnet --fund
```

3. Install app dependencies:

```bash
npm install
npm run db:generate
npm --workspace backend run db:migrate
npm run dev
```

4. Build and deploy the contract:

```bash
cd contract
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/teachfund_stream_contract.wasm --source alice --network testnet -- --admin alice --asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --project_name "TeachFund Stream"
```

5. Use Freighter on testnet for browser signing. Always poll transaction finality.
