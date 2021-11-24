#!/bin/bash

docker run -it --rm \
    -v $(pwd)/docker:/root/docker \
    -p 26657:26657 \
    -p 26656:26656 \
    -p 1337:1337 \
    --name secretdev \
    --entrypoint /root/docker/chain-setup.sh \
    enigmampc/secret-network-sw-dev

