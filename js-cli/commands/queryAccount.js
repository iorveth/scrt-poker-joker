const { CosmWasmClient } = require("secretjs");


const queryAccount = async (player) => {
  const httpUrl = process.env.SECRET_REST_URL;
  const queryClient = new CosmWasmClient(httpUrl, false);
  const playerAddr = `PLAYER${player}_ADDR`;
  const addr = process.env[playerAddr];
  const account = await queryClient.getAccount(addr)
  console.log('Account info: ', account);
};

module.exports = queryAccount;
