const { CosmWasmClient } = require("secretjs");

const conf = new (require("conf"))();

const queryOwnerNft = async () => {
  const httpUrl = process.env.SECRET_REST_URL;
  const queryClient = new CosmWasmClient(httpUrl, false);

  const nftAddr = conf.get("nftAddr");
  const player1NftId = conf.get('player1NftId');
  if (nftAddr.length == 0) {
    console.log("no nft contract address in local state");
    return;
  }

	console.log(player1NftId)
	let queryMsg = { nft_info: { token_id:  player1NftId } };
  let player1Nft = await queryClient.queryContractSmart(nftAddr, queryMsg);

  console.log("NFT ", JSON.stringify(player1Nft));
  conf.set("player1Nft", player1Nft);
};

module.exports = queryOwnerNft;
