const { CosmWasmClient } = require("secretjs");

const conf = new (require("conf"))();

const queryOwnerNft = async () => {
  const httpUrl = process.env.SECRET_REST_URL;
  const queryClient = new CosmWasmClient(httpUrl, false);

  const nftAddr = conf.get("nftAddr");
  const player1NftId = conf.get("player1NftId");
  if (nftAddr.length == 0) {
    console.log("no nft contract address in local state");
    return;
  }


  const daoAddr = conf.get("daoAddr");
  const player1Addr = conf.get("player1Addr");
  let queryMsg = { player_nfts: { player: player1Addr, viewer: daoAddr } };
  let player1Nft = await queryClient.queryContractSmart(daoAddr, queryMsg);

  console.log("expected nft ID: ", player1NftId);
  console.log("queried  nft ID", JSON.stringify(player1Nft));

  let metadataQueryMsg = { nft_info: { token_id: player1NftId } };
  let player1NftMetadata = await queryClient.queryContractSmart(nftAddr, metadataQueryMsg);
  conf.set("player1Nft", player1NftMetadata);
  console.log("NFT Metadata", JSON.stringify(player1NftMetadata));
};

module.exports = queryOwnerNft;
