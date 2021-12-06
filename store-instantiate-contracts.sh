#!/bin/bash
set -ex 

docker run --rm -v "$(pwd)/pj-dao":/contract \
  enigmampc/secret-contract-optimizer:1.0.5

docker run --rm -v "$(pwd)/pj-nft":/contract \
  enigmampc/secret-contract-optimizer:1.0.5

cd pj-dao/
gzip -d contract.wasm.gz
cd ../pj-nft/
gzip -d contract.wasm.gz
<<<<<<< HEAD
cd ../js-cli
=======
cd ../js-deploy
>>>>>>> 5a157d0 (wip)
node index.js deploy 
