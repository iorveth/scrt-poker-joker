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
- there is a base bet value per dice
- the initial total stake is number of players x base bet x 5 (number of dice per player)

### Play

- each player takes turn to roll all 5 dice in their initial roll
- after each initiall roll, each player can reroll an arbitrary number of die/dices: _r_ (up to 5) once
- if the player decides to reroll, they must place _base bet x r_ to the total stake
- after all players have had their initial (optional reroll), the player with the highest score wins

#### Scoring

- *1 point:* 1 pair
- *2 points:* 2 pairs
- *3 points:* 3 of a kind
- *4 points:* 3 of a kind + 1 pair
- *4 points:* 4 of a kind
- *4 points:* straight
- *5 points:* 5 of a kind

#### Winning

The winner of the game takes all the stakes,
and their dice NFT will increase xp and thus allowing the owner to _TBC_

