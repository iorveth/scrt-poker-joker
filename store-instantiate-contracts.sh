#!/bin/bash
set -ex 

docker run --rm -v "$(pwd)/pj-dao":/contract \
  enigmampc/secret-contract-optimizer:1.0.5

docker run --rm -v "$(pwd)/pj-nft":/contract \
  enigmampc/secret-contract-optimizer:1.0.5

# This stores and instantiates the contract
docker exec secretdev /root/code/docker/store-contracts.sh
