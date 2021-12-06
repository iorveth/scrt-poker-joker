# scrt-poker-joker

Scrt Poker Joker is a multiplayer dice game implemented on the Scrt Network.
This is inspired by a mini game in the [Sea Dogs] game series.
It is similar, yet more simple than Poker dice.

The players of the game role their own sets of dice, which are represented by Secret NFTs (SNIP-721).
The owner of the dice NFT, depending on the attributes of the NFT, may have different privilledges.

[Sea Dogs]: https://www.gamepressure.com/games/sea-dogs-to-each-his-own/ze52a6

## Poker Joker Rules

### Set up

- the total number of dice per player is 5
- there is a base bet value per die in the game (set by the initiator, more on this later)
- the initial total stake is number of players x base bet x 5 x 2 (number of dice per player x 2 rounds )

### Play

- each player takes turn to roll all 5 dice in their initial roll
- after each initiall roll, each player can reroll an arbitrary number of die/dices: _r_ (up to 5) once
- if the player decides to reroll, they must place _base bet x r_ to the total stake
- after all players have had their initial (optional reroll), the player with the highest score wins

There are two modes for play in scrt poker joker, the clear and shielded mode.

- **Clear mode:** each player does their initial roll and the result of that roll can be observed by all the other players in the game
- **Shielded mode:** the other players cannot see what the shielded player have rolled but only that they have / have not rerolled.
Naturally if a player is using the shielded mode, it is unlikely that an unshielded player will opt in to play.

#### Scoring

- *1 point:* 1 pair
- *2 points:* 2 pairs
- *3 points:* 3 of a kind
- *4 points:* 3 of a kind + 1 pair
- *4 points:* 4 of a kind
- *4 points:* straight (1-5)
- *5 points:* 5 of 1s 

#### Winning

The winner of the game takes all the stakes,
and their dice NFT will increase xp and thus allowing the owner to access different privilledges.

## Dice NFT levels

In order to align the value of the Dice NFT with their utility, we have initially set up some privilledges below:

1. **Base Bet**: The base bet of a game is set by any player, however, there is a maximum amount depending on the initiator's NFT `xp`. Please see [xp table] for details
2. **Shielded Game**: Player with the high `xp` NFT can play in the shielded mode

[xp table]: (xp-table)

### XP Table

| Point | Base Bet | Access to Shielded Game |
| ----- | ------   | -------       |
| < 100 | 1 Scrt   | No            |
| 100 < 200 | 2 Scrt   | No            |
| 200 < 400 | 4 Scrt   | Yes |
| 400 + | 8 Scrt   | Yes |


## Useful links

- https://build.scrt.network/
- https://github.com/scrtlabs/SecretJS-Templates
- https://github.com/baedrik/snip721-reference-impl
- https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-721.md
- https://build.scrt.network/dev/developing-secret-contracts.html#debug-printing


## DAO-like voting (TBC)
With the Dice NFT, owners can take part in the governance of the game.


## Ideas to explore
- loser loses points

