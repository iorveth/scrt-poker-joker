const {
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const conf = new (require("conf"))();
const customFees = require("../util.js");

const collateralise = async (player, tokenId, priceDenom, priceAmount) => {
  const httpUrl = process.env.SECRET_REST_URL;
  const playerMnemonic = `PLAYER${player}_MNEMONIC`;
  const player1 = process.env[playerMnemonic];
  const signingPen1 = await Secp256k1Pen.fromMnemonic(player1);
  const pubkey1 = encodeSecp256k1Pubkey(signingPen1.pubkey);
  const accAddress1 = pubkeyToAddress(pubkey1, "secret");
  const txEncryptionSeed1 = EnigmaUtils.GenerateNewSeed();
  const player1Client = new SigningCosmWasmClient(
    httpUrl,
    accAddress1,
    (signBytes) => signingPen1.sign(signBytes),
    txEncryptionSeed1,
    customFees
  );

  const nftAddr = conf.get("nftAddr");

  const priceCoin = { denom: priceDenom, amount: priceAmount };
  const collateralMsg = {
    collateralise: {
      token_id: tokenId,
    },
  };
  let r = await player1Client.execute(nftAddr, collateralMsg, "", [priceCoin]);
  console.log("result: ", JSON.stringify(r, null, 4));
};
module.exports = collateralise;
