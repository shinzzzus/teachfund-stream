#!/usr/bin/env bash
set -euo pipefail
cd contract
stellar contract invoke --id ${CONTRACT_ID:?CONTRACT_ID required} --source ${SOURCE:-alice} --network ${NETWORK:-testnet} -- total_locked
