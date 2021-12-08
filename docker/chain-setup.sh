#!/bin/bash

# THIS IS FOR HACKTHON PURPOSE ONLY AND SHOULD NOT BE REUSED
ADMIN_ADDR="secret1hc269crdqnu88gqqpwkens3rwna5522juzenn8"
PLAYER1_ADDR="secret15lj543ptx9eqr42hwd4770ctyy3axmqn0sdleq"
PLAYER2_ADDR="secret152q637p32qg6qe76u3yxhnww47cd0zcc63y53t"
PLAYER3_ADDR="secret108pxrmtl36t0cd9k2eh8k9qae7ctxxfamp473u"
PLAYER4_ADDR="secret18qr6dqal3399y7j08eqqrq0lnpyflgdpzykypj"
PLAYER5_ADDR="secret1pxuz4dad4esz82awvlpjsv90z55tysr5ywpja4"
PLAYER6_ADDR="secret1032z04v73yvgy9mmdzx33jyxvc8055gkc0zevk"
PLAYER7_ADDR="secret1kcnhuj80l8xcqz2kym0hkkuhsken306yyhwj9p"
PLAYER8_ADDR="secret1375krvha8d50scrtdelwhe9acaxqjj55azmy3v"
PLAYER9_ADDR="secret1fpsgq2ruw30h9wpzrxdfk6f79c49af6qdv5eat"
PLAYER10_ADDR="secret1jsk8zhynmv3acwr88v438w5wv04mwcdw3x3yte"

file=~/.secretd/config/genesis.json
if [ ! -e "$file" ]
then
  # init the node
  rm -rf ~/.secretd/*
  rm -rf /opt/secret/.sgx_secrets/*

  if [ -z "${CHAINID}" ]; then
    chain_id="$CHAINID"
  else
    chain_id="secretdev-1"
  fi

  mkdir -p ./.sgx_secrets
  secretd config chain-id "$chain_id"
  secretd config output json
  secretd config keyring-backend test

  # export SECRET_NETWORK_CHAIN_ID=secretdev-1
  # export SECRET_NETWORK_KEYRING_BACKEND=test
  secretd init banana --chain-id "$chain_id"


  cp ~/node_key.json ~/.secretd/config/node_key.json
  perl -i -pe 's/"stake"/ "uscrt"/g' ~/.secretd/config/genesis.json
  perl -i -pe 's/"172800000000000"/"90000000000"/g' ~/.secretd/config/genesis.json # voting period 2 days -> 90 seconds

 secretd keys add a
  secretd keys add b
  secretd keys add c
  secretd keys add d

  secretd add-genesis-account $ADMIN_ADDR 20000000000000000000000000uscrt
  secretd add-genesis-account $PLAYER1_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER2_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER3_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER4_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER5_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER6_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER7_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER8_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER9_ADDR 200000000uscrt
  secretd add-genesis-account $PLAYER10_ADDR 2000000uscrt
  secretd add-genesis-account "$(secretd keys show -a a)" 1000000000000000000uscrt
  secretd add-genesis-account "$(secretd keys show -a b)" 1000000000000000000uscrt
#  secretd add-genesis-account "$(secretd keys show -a c)" 1000000000000000000uscrt
#  secretd add-genesis-account "$(secretd keys show -a d)" 1000000000000000000uscrt


  secretd gentx a 1000000uscrt --chain-id "$chain_id"
  secretd gentx b 1000000uscrt --chain-id "$chain_id"
#  secretd gentx c 1000000uscrt --keyring-backend test
#  secretd gentx d 1000000uscrt --keyring-backend test

  secretd collect-gentxs
  secretd validate-genesis

#  secretd init-enclave
  secretd init-bootstrap
#  cp new_node_seed_exchange_keypair.sealed .sgx_secrets
  secretd validate-genesis
fi

lcp --proxyUrl http://localhost:1317 --port 1337 --proxyPartial '' &

# sleep infinity
source /opt/sgxsdk/environment && RUST_BACKTRACE=1 secretd start --rpc.laddr tcp://0.0.0.0:26657 --bootstrap &

gunicorn --bind 0.0.0.0:5000 svc 
