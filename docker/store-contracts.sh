#!/bin/bash
set -ex

secretd tx compute store /root/code/pj-dao/contract.wasm.gz --from a --gas 3000000 -y --keyring-backend test
sleep 5 
secretd tx compute store /root/code/pj-nft/contract.wasm.gz --from a --gas 3000000 -y --keyring-backend test
sleep 5 
RES=$(secretd query compute list-code)
DAO_CODE_ID=$(echo $RES | jq -r '.[-2].id')
NFT_CODE_ID=$(echo $RES | jq -r '.[-1].id')

INIT='{}'
secretd tx compute instantiate $DAO_CODE_ID "$INIT" --from a --label "pj-dao-$DAO_CODE_ID" -y --keyring-backend test
sleep 5 
secretd tx compute instantiate $NFT_CODE_ID "$INIT" --from b --label "pj-nft-$NFT_CODE_ID" -y --keyring-backend test
sleep 5 
DAO_ADDRESS=$(secretd query compute list-contract-by-code $DAO_CODE_ID)
NFT_ADDRESS=$(secretd query compute list-contract-by-code $NFT_CODE_ID) 

echo "DAO_ADDRESS: $(DAO_ADDRESS)"
echo "NFT_ADDRESS: $(NFT_ADDRESS)"
