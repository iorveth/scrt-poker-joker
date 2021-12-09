const {
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const conf = new (require("conf"))();
const customFees = require("../util.js");

const queryOwnerNft = async (player) => {
  const httpUrl = process.env.SECRET_REST_URL;
  const playerMnemonic = `PLAYER${player}_MNEMONIC`;
  const player1 = process.env[playerMnemonic];
  const signingPen1 = await Secp256k1Pen.fromMnemonic(player1);
  const pubkey1 = encodeSecp256k1Pubkey(signingPen1.pubkey);
  const accAddress1 = pubkeyToAddress(pubkey1, "secret");
  const txEncryptionSeed1 = EnigmaUtils.GenerateNewSeed();
  const client = new SigningCosmWasmClient(
    httpUrl,
    accAddress1,
    (signBytes) => signingPen1.sign(signBytes),
    txEncryptionSeed1,
    customFees
  );

  const nftAddr = conf.get("nftAddr");

  if (nftAddr.length == 0) {
    console.log("no nft contract address in local state");
    return;
  }

  const chainId = await client.getChainId();
  const permitName = "A cool Secret NFT game";
  const permissions = ["owner"];
  const allowedTokens = [nftAddr];
  const fee = {
    amount: [
      {
        denom: "uscrt",
        amount: "0",
      },
    ],
    gas: "1",
  };

  const signature = await client.signAdapter(
    [
      {
        type: "query_permit",
        value: {
          permit_name: permitName,
          allowed_tokens: allowedTokens,
          permissions: permissions,
        },
      },
    ],
    fee,
    chainId,
    "",
    0,
    0
  );

  const token_list = await client.queryContractSmart(nftAddr, {
    with_permit: {
      query: { tokens: { owner: accAddress1, start_after: null, limit: null } },
      permit: {
        params: {
          permit_name: permitName,
          allowed_tokens: allowedTokens,
          chain_id: chainId,
          permissions: permissions,
        },
        signature: {
          pub_key: pubkey1,
          signature: signature.signatures[0].signature,
        },
      },
    },
  });

  token_list.token_list.tokens.forEach(async (value) => {
    let metadataQueryMsg = { nft_info: { token_id: value } };
    let player1NftMetadata = await client.queryContractSmart(
      nftAddr,
      metadataQueryMsg
    );
    console.log("NFT Metadata", JSON.stringify(player1NftMetadata, null, 4));
  });
};

module.exports = queryOwnerNft;
