# Stellar Fullstack Cheatsheet Applied

- Next.js 15 TypeScript app with client-side Freighter calls via dynamic import.
- Soroban RPC: https://soroban-testnet.stellar.org
- XLM SAC: CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
- Transaction flow: simulate, sign, submit, poll for finality.
- Backend exposes /api/health, /api/records, and /api/payments/quote.
- Database uses Prisma SQLite for local operational records.
