.PHONY: build test deploy invoke clean

NETWORK ?= testnet
SOURCE ?= alice
WASM = target/wasm32v1-none/release/teachfund_stream_contract.wasm

build:
	stellar contract build

test:
	cargo test

deploy: build
	stellar contract deploy --wasm $(WASM) --source $(SOURCE) --network $(NETWORK) -- --admin $(SOURCE) --asset CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --project_name "TeachFund Stream"

invoke:
	stellar contract invoke --id $(CONTRACT_ID) --source $(SOURCE) --network $(NETWORK) -- total_locked

clean:
	cargo clean
