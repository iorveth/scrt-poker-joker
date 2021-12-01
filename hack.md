# Hacking

## Setup

Pulling and setting the submodule

1. `git submodule update --remote --recursive`

Run the local node:

1. make sure you have `local-node-setup.sh`, `store-instantiate-contracts.sh` and `docker/*.sh` as executables
1. run `./local-node-setup.sh `
1. run `./store-instantiate-contracts.sh`

This will provide you with Admin and 10 players with chain balance in uscrt

### Params

| player | pj-nft-xp | scrt balance |
| ------ | --------- | ------------ |
| 1      | 0         | 1000         |
| 2      | 100       | 2000         |
| 3      | 200       | 2000         |
| 4      | 300       | 1000         |
| 5      | 300       | 10000        |
| 6      | 400       | 10000        |
| 7      | 400       | 10000        |
| 8      | 500       | 10000        |
| 9      | 0         | 0            |

---

## Demo

### Set up

1. Run local chain
1. Store contracts
1. Instantiate pj-nft contract with account `admin`
   1. Initially NFT distribution 8 different NFTs at different levels belongs to different 8 accounts
   - `admin` Call `BatchMintNFT` with the set metadata
   - DAO calls `MintNFTClones` (for a few NFTs so new users can have some)
1. Instantiate pj-dao contract with account `admin`
   1. Admin sends in scrt to pj DAO
   1. Provide the same 8 accounts (as NFT contract) with pjc
   1. Initialise to use the pj-nft contract address

### Demo

1. Describe rules
1. Player 9 arrives on landing page
   1. Connect to wallet
   1. New Player page (button to start playing)
   1. DAO tx() -> `joinDao()` -> call back: on success redirect to user home page
   1. Dao contract mints NFT to player 9
   1. NFT display in player's inventory
1. (optional) Player 9 attempts to starts game
   1. button to start game ()
   1. DAO tx() -> `CreateNewGameRoom` -> call back: `Error::not_enough_scrt`
   1. popup? button to collateralise
   1. NFT tx() -> `collateral_init` -> pending
1. (optional) manually provide collateral (nodejs script?)
1. Player 9 starts game again -> success
1. Player 1 arrives on landing page
   1. Connect to wallet
   1. Query: Sees active games (the one with Player 9)
   1. Query: Sees own NFTs
   1. joins player 9 with `join` game with a selected dice set
   1. DAO tx() -> `joinGame(nft_id)` -> call back: `Success (game_id)` -> redirect to game page
1. Player 1 and Player 9 roll dice - button
   1. DAO tx() -> `roll(game_id)` -> call back: `dice outcome {}`
   1. Query: display dice outcome to both parties
1. Reroll
   1. DAO tx() -> `reroll(game_id, number_of_dic)` -> `dice outcome {}`
1. End game
   1. DAO tx() -> `endGame(game_id)` -> `pending_other_player` / `game ended`
   1. if game ended -> home page
1. Show balance update
   - if Player 9 loses, DAO will exercise the transfers `endGame()` should trigger check collateral

#### pages

1. Landing page
   1. All active / waiting games
   1. Join Dao button
1. Join Dao popup?
   1. join Dao button
1. Home page
   1. see all games
   1. join game
   1. start new game
   1. user balance
   1. user dice set nfts
1. game page
   1. see what the other person has rolled
   1. see own dice metadata and info (query blockchain for metadata)
   1. reroll button after 1st roll

#### DAO contract msg / queries

1. joinDao
1. CreateNewGameRoom
1. joinGame
1. roll
1. reroll
1. endGame

1. getActiveGames

#### NFT contract msg / queries

1. mint
1. setMetadata (from DAO)
1. collateral_init
1. collateralise
1. uncollateralise

1. getNFTs (dice level, colours, )
