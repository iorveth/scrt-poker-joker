const {
  CosmWasmClient,
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const conf = new (require("conf"))();
const customFees = require("../util.js");

const joinDao = async (player, tokenId, viewingKey) => {
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

  const daoAddr = conf.get("daoAddr");
  console.log("dao address: ", daoAddr);
  const joinDaoMsg = { join_dao: { nft: null } };
  let r = await player1Client.execute(daoAddr, joinDaoMsg);
  console.log("joined Dao and mint: ", JSON.stringify(r, null, 4));
};
module.exports = joinDao;
