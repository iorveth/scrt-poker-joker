const { CosmWasmClient } = require("secretjs");

const conf = new (require("conf"))();

const queryOwnerNft = async (player) => {
  const httpUrl = process.env.SECRET_REST_URL;
  const queryClient = new CosmWasmClient(httpUrl, false);

  const nftAddr = conf.get("nftAddr");
  if (nftAddr.length == 0) {
    console.log("no nft contract address in local state");
    return;
  }

  const daoAddr = conf.get("daoAddr");
  const player1Addr = conf.get(`player${player}Addr`);
  let queryMsg = { player_nfts: { player: player1Addr, viewer: daoAddr } };
  let player1Nft = await queryClient.queryContractSmart(daoAddr, queryMsg);

  console.log("queried  nft ID", JSON.stringify(player1Nft, null, 4));
  player1Nft.forEach(async (value) => {
    let metadataQueryMsg = { nft_info: { token_id: value } };
    let player1NftMetadata = await queryClient.queryContractSmart(
      nftAddr,
      metadataQueryMsg
    );
    console.log("NFT Metadata", JSON.stringify(player1NftMetadata, null, 4));
  });
};

module.exports = queryOwnerNft;
