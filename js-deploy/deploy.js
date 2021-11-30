const {
    EnigmaUtils, SigningCosmWasmClient, Secp256k1Pen, pubkeyToAddress, encodeSecp256k1Pubkey
} = require("secretjs");
const fs = require('fs');

require('dotenv').config();
dotenv.config({ path: `${__dirname}/../.env.dev`});

function getContract(path) {
  return fs.readFileSync(path);
}

const main = async () => {
    
    const mnemonic = process.env.ADMIN_MNEMONIC;
    const httpUrl = process.env.SECRET_REST_URL;
    const signingPen = await Secp256k1Pen.fromMnemonic(mnemonic);
    const pubkey = encodeSecp256k1Pubkey(signingPen.pubkey);
    const accAddress = pubkeyToAddress(pubkey, 'secret');

    const txEncryptionSeed = EnigmaUtils.GenerateNewSeed();
    const fees = {
        send: {
            amount: [{ amount: "80000", denom: "uscrt" }],
            gas: "80000",
        },
    }

    const daoContract = getContract("../target/")
    const client = new SigningCosmWasmClient(
        httpUrl,
        accAddress,
        (signByes) => signingPen.sign(signBytes), 
        txEncryptionSeed, fees
    );
    const rcpt = accAddress; // Set recipient to sender for testing

    //optional memo
    const memo = 'sendTokens example';

    const sent = await client.sendTokens(rcpt, [{amount: "1234", denom: "uscrt"}], memo)
    console.log('sent', sent)

    // Query the tx result
    const query = {id: sent.transactionHash}
    const tx = await client.searchTx(query)
    console.log('Transaction: ', tx);
}

main().then(resp => {
    console.log(resp);
}).catch(err => {
    console.log(err);
})
