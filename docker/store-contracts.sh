#!/bin/bash
set -ex

secretd tx compute store /root/code/pj-dao/contract.wasm.gz --from a --gas 30000000 -y --keyring-backend test
secretd tx compute store /root/code/pj-nft/contract.wasm.gz --from b --gas 30000000 -y --keyring-backend test
sleep 5 
RES=$(secretd query compute list-code)
DAO_CODE_ID=$(echo $RES | jq -r '.[-2].id')
NFT_CODE_ID=$(echo $RES | jq -r '.[-1].id')

DAO_INIT='{}'
NFT_INIT="{\"name\": \"PokerJokerDiceNFT\", \"symbol\": \"pjx\", \"admin\": "$(secretd keys show -a b)", \"entropy\": "hello", \"royalty_info\": "", \"config\": "", \"post_init_callback\": ""}"

# pub struct InitMsg {
#     /// name of token contract
#     pub name: String,
#     /// token contract symbol
#     pub symbol: String,
#     /// optional admin address, env.message.sender if missing
#     pub admin: Option<HumanAddr>,
#     /// entropy used for prng seed
#     pub entropy: String,
#     /// optional royalty information to use as default when RoyaltyInfo is not provided to a
#     /// minting function
#     pub royalty_info: Option<RoyaltyInfo>,
#     /// optional privacy configuration for the contract
#     pub config: Option<InitConfig>,
#     /// optional callback message to execute after instantiation.  This will
#     /// most often be used to have the token contract provide its address to a
#     /// contract that instantiated it, but it could be used to execute any
#     /// contract
#     pub post_init_callback: Option<PostInitCallback>,
# }

secretd tx compute instantiate $DAO_CODE_ID "$DAO_INIT" --from a --label "pj-dao-$DAO_CODE_ID" -y --gas 3000000 --keyring-backend test
secretd tx compute instantiate $NFT_CODE_ID "$NFT_INIT" --from b --label "pj-nft-$NFT_CODE_ID" -y --gas 3000000 --keyring-backend test
sleep 5 

DAO_ADDRESS=$(secretd query compute list-contract-by-code $DAO_CODE_ID)
NFT_ADDRESS=$(secretd query compute list-contract-by-code $NFT_CODE_ID) 

echo "DAO_ADDRESS: $DAO_ADDRESS"
echo "NFT_ADDRESS: $NFT_ADDRESS"

