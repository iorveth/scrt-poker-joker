#!/bin/bash
set -ex
secretd tx compute store /root/pj-dao/contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test
secretd tx compute store /root/pj-nft/contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test
