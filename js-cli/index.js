#! /usr/bin/env node

const { program } = require('commander')
const deploy = require("./commands/deploy.js")
const joinDao = require("./commands/joinDao.js")
const queryOwnerNft = require("./commands/queryOwnerNft.js")
const transfer = require("./commands/transfer.js")
const adminMint = require("./commands/adminMint.js")
const queryAccount = require("./commands/queryAccount.js")
const initCollateral= require("./commands/initCollateral.js")
const collateralise= require("./commands/collateralise.js")
const unCollateralise= require("./commands/unCollateralise.js")

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
    .command('queryrNftOf <player>')
    .description('queries the owner nft')
    .action(queryOwnerNft)

program
    .command('queryAccountOf <player>')
    .description('query account')
    .action(queryAccount)

program
    .command('adminMint <to>')
    .description('admin mint for')
    .action(adminMint)

program
    .command('initCollateral <from> <tokenId> <priceDenom> <priceAmount> <replaymentAmount> [expiration]')
    .description('init collateral by current owner')
    .action(initCollateral)

program
    .command('collateralise <from> <tokenId> <priceDenom> <priceAmount>')
    .description('offers to collateralise')
    .action(collateralise)

program
    .command('unCollateralise <from> <tokenId> [priceDenom] [priceAmount]')
    .description('either make repayment or take NFT after expiration')
    .action(unCollateralise)

// For dev
program
    .command('transfer <to> <amount> <denom>')
    .description('admin transfer ')
    .action(transfer)

program.parse();
