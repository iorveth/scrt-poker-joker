#!/bin/bash
set -ex 

set -o allexport
source .env.dev
set -a allexport

docker run --rm -v "$(pwd)":/contract \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer

# docker exec mycontainer /bin/sh -c "cmd1;cmd2;...;cmdn"

# INIT='{"purchase_price":{"amount":"100","denom":"ucosm"},"transfer_price":{"amount":"999","denom":"ucosm"}}'
# # deploy tutorial contract
# RES=$(wasmd tx wasm store artifacts/cw_nameservice.wasm \
#     --from=wallet \
#     $KEYRING $KEYDIR \
#     --node=$RPC \
#     --chain-id=$CHAIN_ID \
#     --gas-prices=0.1ucosm \
#     --gas=auto \
#     --gas-adjustment=1.3 \
#     --home=$APP_HOME -y)
# 
# # initialise contract
# CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
# wasmd tx wasm instantiate $CODE_ID $INIT \
#     --from wallet \
#     --label "awesome name service" \
#     --node=$RPC \
#     --chain-id=$CHAIN_ID \
#     --gas-prices=0.1ucosm \
#     --gas=auto \
#     --gas-adjustment=1.3\
#     --home=$APP_HOME \
#     $KEYRING $KEYDIR -y 
