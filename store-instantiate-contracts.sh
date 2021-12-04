#!/bin/bash
set -ex 

docker run --rm -v "$(pwd)/pj-dao":/contract \
  enigmampc/secret-contract-optimizer:1.0.5

# docker run --rm -v "$(pwd)/pj-nft":/contract \
#   enigmampc/secret-contract-optimizer:1.0.5

cd pj-dao/
gzip -d contract.wasm.gz
cd ../pj-nft/
gzip -d contract.wasm.gz
# cd ../js-docker
# node deploy.js
