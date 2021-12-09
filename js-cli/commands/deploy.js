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
    "PokerJokerDAO" +
      Math.ceil(Math.random() * 10000, "some memo", [
        {
          denom: "uscrt",
          amount: String(1_000_000),
        },
      ])
  );

  const daoAddr = daoContract.contractAddress;
  console.log("instantiated dao contract: ", daoAddr);

  // looks like instantiate sending fund does not work
  await signClient.sendTokens(daoAddr, [
    { amount: String(1_000_000_000), denom: "uscrt" },
  ]);
  const daoAccount = await signClient.getAccount(daoAddr);
  console.log("daoAccount: ", daoAccount);
  conf.set("daoAddr", daoAddr);

  // ---- Use Admin client to instantiate NFT via DAO ----
  const createContractMsg = {
    create_nft_contract: {},
  };

  try {
    await signClient.execute(daoContract.contractAddress, createContractMsg);
  } catch (e) {
    console.log(e);
  }

  // ----  Query DAO to get NFT contract address  and info ----
  console.log("Querying dao contract for nft contract address");
  const nftAddr = await signClient.queryContractSmart(daoAddr, {
    nft_address: {},
  });
  console.log(`nftAddress: ${nftAddr}`);
  conf.set("nftAddr", nftAddr);

  await signClient.sendTokens(nftAddr, [
    { amount: String(1_000_000_000), denom: "uscrt" },
  ]);
  const nftAccount = await signClient.getAccount(nftAddr);
  console.log("nftAccount: ", nftAccount);

  console.log(
    "Querying nft contract for contract info to ensure address is correct"
  );
  const nftInfo = await signClient.queryContractSmart(nftAddr, {
    contract_info: {},
  });

  console.log(`nftInfo: `, nftInfo);
};

module.exports = deploy;
