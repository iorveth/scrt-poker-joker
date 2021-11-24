# Hacking  

## Setup

Run the local node: 
1. make sure you have `local-node-setup.sh` and `docker/chain-setup.sh` as executables
2. run  `./local-node-setup.sh `


This will provide you with Admin and 10 players with chain balance in uscrt

### Params

| player | pj-nft-xp | pjc balance |
| ---    | ---       | ----        |
| 1      | 0         | 1000        |
| 2      | 100       | 2000        |
| 3      | 200       | 2000        |
| 4      | 300       | 1000        |
| 5      | 300       | 10000       |
| 6      | 400       | 10000       |
| 7      | 400       | 10000       |
| 8      | 500       | 10000       |
| 9      | 0         | 0           |


___

## Demo

### Set up
1. Run local chain
1. Store contracts
1. Instantiate pj-nft contract with account `admin` 
    1. Initially NFT distribution 10 different NFTs at different levels belongs to different 10 accounts
1. Instantiate pj-dao contract with account `admin`
    1. Initialise with 10000000 pjc (pj coins)
    1. Provide the same 10 accounts (as NFT contract) with pjc
    1. Initialise to use the NFT contract address as members

### Demo
1. Describe rules
1. Player 9 arrives and mints dice NFT (0 pj-NFT, 0 pjc)
1. Player 9 collateralise the NFT and plays first game 
1. Player 1 has plays with player 9
1. show balance update
1. Shielded Game: Player 7 starts
1. Shielded Game: Player 8 joins 
1. show balance update

