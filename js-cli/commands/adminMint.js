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

const adminMint = async (to) => {
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

	const daoAddr = conf.get("daoAddr");
  const playerAddr = conf.get(`player${to}Addr`);
	const admintMintMsg= { admin_mint: { to: playerAddr, private_metadata : null } };
  let r = await client.execute(daoAddr, admintMintMsg);
  console.log(`Admin Minted for : ${to}`, JSON.stringify(r));
};
module.exports = adminMint;
