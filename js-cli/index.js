#! /usr/bin/env node

const { program } = require('commander')
const deploy = require("./commands/deploy.js")
const joinDao = require("./commands/joinDao.js")
const queryOwnerNft = require("./commands/queryOwnerNft.js")

// Load environment variables
require("dotenv").config({ path: `${__dirname}/../.env.dev` });

program
    .command('deploy')
    .description('Store and instantiate Game Dao')
    .action(deploy)

program
    .command('joinDao')
    .description('lets player join the dao')
    .action(joinDao)

program
    .command('queryOwnerNft')
    .description('queries the owner nft')
    .action(queryOwnerNft)

program.parse();
