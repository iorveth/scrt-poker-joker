const {
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const fs = require("fs");
const conf = new (require("conf"))();
const customFees = require("../util.js");

const deploy = async () => {
  //  ---- Creating Admin wallet for contract deployment (store + instantiate) ----
  const httpUrl = process.env.SECRET_REST_URL;
  const mnemonic = process.env.ADMIN_MNEMONIC;
  const signingPen = await Secp256k1Pen.fromMnemonic(mnemonic);
  const pubkey = encodeSecp256k1Pubkey(signingPen.pubkey);
  const accAddress = pubkeyToAddress(pubkey, "secret");
  const txEncryptionSeed = EnigmaUtils.GenerateNewSeed();
  const signClient = new SigningCosmWasmClient(
    httpUrl,
    accAddress,
    (signBytes) => signingPen.sign(signBytes),
    txEncryptionSeed,
    customFees
  );

  // ---- Use Admin client to upload contracts ----
  const daoWasm = fs.readFileSync("../pj-dao/contract.wasm");
  let uploadReceipt = await signClient.upload(daoWasm, {});
  const daoCodeId = uploadReceipt.codeId;
  console.log("uploaded dao wasm: ", daoCodeId);

  const nftWasm = fs.readFileSync("../pj-nft/contract.wasm");
  uploadReceipt = await signClient.upload(nftWasm, {});
  const nftCodeId = uploadReceipt.codeId;
  console.log("uploaded nft wasm: ", nftCodeId);

  // ---- Use Admin client to instantiate DAO ----
  const nftContractCodeHash = await signClient.restClient.getCodeHashByCodeId(
    nftCodeId
  );
  const daoInitMsg = {
    nft_code_id: nftCodeId,
    nft_code_hash: nftContractCodeHash,
  };
  const daoContract = await signClient.instantiate(
    daoCodeId,
    daoInitMsg,
    "PokerJokerDAO" + Math.ceil(Math.random() * 10000)
  );

  const daoAddr = daoContract.contractAddress;
  conf.set("daoAddr", daoAddr);
  console.log("instantiated dao contract: ", daoAddr);

  // ---- Use Admin client to instantiate NFT via DAO ----
  const createContractMsg = {
    create_nft_contract: {},
  };

  try {
    await signClient.execute(daoContract.contractAddress, createContractMsg);
  } catch (e) {
    console.log("probably already deployed, nvm");
    console.log(e);
  }

  // ----  Query DAO to get NFT contract address  and info ----
  console.log("Querying dao contract for nft contract address");
  const nftAddr = await signClient.queryContractSmart(daoAddr, {
    nft_address: {},
  });
  console.log(`nftAddress: ${nftAddr}`);
  conf.set("nftAddr", nftAddr);

  console.log(
    "Querying nft contract for contract info to ensure address is correct"
  );
  const nftInfo = await signClient.queryContractSmart(nftAddr, {
    contract_info: {},
  });

  console.log(`nftInfo: `, nftInfo);
};

module.exports = deploy;
