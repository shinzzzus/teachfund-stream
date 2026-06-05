# Contract Design

TeachFund Stream follows the three-part Soroban guide:

1. Struct + impl: `contract/src/lib.rs`
2. Storage: `contract/src/storage.rs`
3. Errors + types: `contract/src/errors.rs` and `contract/src/types.rs`

## Domain API

- `open_course`: open a course grant.
- `fund_course`: transfer XLM SAC into contract escrow.
- `verify_completion`: admin/verifier records score and status.
- `release_grant`: transfer approved funds out of contract escrow.

Storage TTL is extended for both instance and persistent keys.
