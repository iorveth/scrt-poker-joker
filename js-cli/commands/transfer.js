const {
  CosmWasmClient,
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const customFees = require("../util.js");

const transfer = async (to, amount, denom) => {
  const httpUrl = process.env.SECRET_REST_URL;
  const signingPen = await Secp256k1Pen.fromMnemonic(process.env.ADMIN_MNEMONIC);
  const pubkey = encodeSecp256k1Pubkey(signingPen.pubkey);
  const accAddress = pubkeyToAddress(pubkey, "secret");
  const txEncryptionSeed = EnigmaUtils.GenerateNewSeed();
  const client = new SigningCosmWasmClient(
    httpUrl,
    accAddress,
    (signBytes) => signingPen.sign(signBytes),
    txEncryptionSeed,
    customFees
  );

  let r = await client.sendTokens(to, [{amount, denom}]);
  console.log("send result: ", r);
  const toAccount = await client.getAccount(to)
  console.log('Receiver Account: ', toAccount);
};

module.exports = transfer;
