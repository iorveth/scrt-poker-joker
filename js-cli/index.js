#! /usr/bin/env node

const { program } = require('commander')
const deploy = require("./commands/deploy.js")
const joinDao = require("./commands/joinDao.js")
const queryOwnerNft = require("./commands/queryOwnerNft.js")
const transfer = require("./commands/transfer.js")

// Load environment variables
require("dotenv").config({ path: `${__dirname}/../.env.dev` });

program
    .command('deploy')
    .description('Store and instantiate Game Dao')
    .action(deploy)

program
    .command('joinDao <player> [tokenId] [viewingKey]')
    .description('lets player join the dao')
    .action(joinDao)

program
    .command('queryPlayerNft <player>')
    .description('queries the owner nft')
    .action(queryOwnerNft)

program
    .command('transfer <to> <amount> <denom>')
    .description('test transfer ')
    .action(transfer)

program
    .command('adminMint <for>')
    .description('admin mint for')
    .action(transfer)

program.parse();
