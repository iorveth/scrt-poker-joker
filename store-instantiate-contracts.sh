#!/bin/bash
set -ex 

set -o allexport
source .env.dev
set -a allexport

#docker run --rm -v "$(pwd)/pj-dao":/contract \
#  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#  enigmampc/secret-contract-optimizer:1.0.5
#
#docker run --rm -v "$(pwd)/pj-nft":/contract \
#  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
#  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#  enigmampc/secret-contract-optimizer:1.0.5

# This stores and instantiates the contract
docker exec secretdev /root/code/docker/store-contracts.sh
