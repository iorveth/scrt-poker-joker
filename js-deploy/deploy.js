const {
  CosmWasmClient,
  EnigmaUtils,
  Secp256k1Pen,
  SigningCosmWasmClient,
  pubkeyToAddress,
  encodeSecp256k1Pubkey,
} = require("secretjs");

const fs = require("fs");

// Load environment variables
require("dotenv").config({ path: `${__dirname}/../.env.dev` });

const customFees = {
  upload: {
    amount: [{ amount: "20000000", denom: "uscrt" }],
    gas: "20000000",
  },
  init: {
    amount: [{ amount: "5000000", denom: "uscrt" }],
    gas: "5000000",
  },
  exec: {
    amount: [{ amount: "500000", denom: "uscrt" }],
    gas: "500000",
  },
  send: {
    amount: [{ amount: "80000", denom: "uscrt" }],
    gas: "80000",
  },
};

const main = async () => {
  const httpUrl = process.env.SECRET_REST_URL;

  // Use key created in tutorial #2
  const mnemonic = process.env.ADMIN_MNEMONIC;

  // A pen s the most basic tool you can think of for signing.
  // This wraps a single keypair and allows for signing.
  const signingPen = await Secp256k1Pen.fromMnemonic(mnemonic);

  // Get the public key
  const pubkey = encodeSecp256k1Pubkey(signingPen.pubkey);

  // get the wallet address
  const accAddress = pubkeyToAddress(pubkey, "secret");
  const txEncryptionSeed = EnigmaUtils.GenerateNewSeed();
  const signClient = new SigningCosmWasmClient(
    httpUrl,
    accAddress,
    (signBytes) => signingPen.sign(signBytes),
    txEncryptionSeed,
    customFees
  );

  console.log(`Admin wallet address=${accAddress}`);

  const daoWasm = fs.readFileSync("../pj-dao/contract.wasm");
  let uploadReceipt = await signClient.upload(daoWasm, {});
  const daoCodeId = uploadReceipt.codeId;
  console.log("uploaded dao wasm: ", daoCodeId);

  const nftWasm = fs.readFileSync("../pj-nft/contract.wasm");
  uploadReceipt = await signClient.upload(nftWasm, {});
  const nftCodeId = uploadReceipt.codeId;
  console.log("uploaded nft wasm: ", nftCodeId);

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
  console.log("instantiated dao contract: ", daoContract);
  const daoAddress = daoContract.contractAddress;

  console.log("instantiating nft contract");
  const createContractMsg = {
    create_nft_contract: {},
  };

  try {
    await signClient.execute(daoContract.contractAddress, createContractMsg);
  } catch (e) {
    console.log("probably already deployed, nvm");
    console.log(e);
  }

  console.log("Querying dao contract for nft contract address");
  const nftAddr = await signClient.queryContractSmart(daoAddress, {
    nft_address: {},
  });
  console.log(`nftAddress: ${nftAddr}`);

  console.log(
    "Querying nft contract for contract info to ensure address is correct"
  );
  const nftInfo = await signClient.queryContractSmart(nftAddr, {
    contract_info: {},
  });

  console.log(`nftInfo: `, nftInfo);

  // ---- Deployment done ----

  console.log("player 1 joins dao and we mint nft for them");
  const player1 = process.env.PLAYER1_MNEMONIC;
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

  // This is basically minting nft for the signer at the moment
  const joinDaoMsg = { join_dao: { nft_id: "" } };
  let r = await player1Client.execute(daoContract.contractAddress, joinDaoMsg);
  console.log("joined Dao and mint: ", JSON.stringify(r));
};

main();
