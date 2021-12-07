
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

module.exports = customFees
