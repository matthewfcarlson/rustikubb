# rustikubb
A rust based rummikub game

The goal is to have a browser based? Rust game.

It is heavily based on https://github.com/snoyberg/rummikub-solver

## Using this project

1. Install cargo-web: https://github.com/koute/cargo-web
2. Run `cargo web start`
2. Open http://localhost:8080

## What is Rummikub?

It's a fun game I played with my grandparents. I was always amazed by the transformations my grandparents could make to the game board. They had a plan and executed, often touching over half the pieces on the board to get it done.

![example of rummikub](https://upload.wikimedia.org/wikipedia/commons/thumb/b/b1/Rummikub_Tiles.jpg/1200px-Rummikub_Tiles.jpg)

This paper : **The complexity of Rummikub Problems** by *Jan N. van Rijn, Frank W. Takes and Jonathan K. Vis* -- http://liacs.leidenuniv.nl/~takesfw/pdf/rummikub.pdf http://liacs.leidenuniv.nl/~takesfw/pdf/rummikub.pdf does a decent job of illustrating Rummikub from an algorithmic sense.

## How to Play

*Here are the rules http://www.rummy-games.com/rules/rummikub.html:*

### Setup

Set all the tilesface downward on the table. Mix them. Each player grabs a tile to see who goes first. The one who picked the highest tile is the first to play. Plays then goes counter clockwise from that first player.

After you determine who plays first, all the players pick 14 tiles and put them into their "hand" or wooden card. You are allowed to look at your own tiles, but not anyone else's. I recommend sorting them to make it easier.

### Starting turn

Before you can place tiles, you need to "break in". You can do this be making an initial meld of at least 50 points using only tiles from your hand. To put it simply, you must put out at least 50 points of tiles that follow the rules and not use any other tiles that have already been played. If you can't do it, draw another tile and skip your turn. 

Tile piles on the board.

#### Runs

All runs must consist of at least 3 tiles of the same color -
```
EXAMPLES:4, 5, 6 red     1, 2, 3, 4 orange
```
#### Sets

Sets must consist of 3 or 4 tiles of different colors
```
EXAMPLE:#4 red, #4 blue, #4 black

Not acceptable: #4 orange, #4 orange, #4 red
```
### SCORING

Each loser adds up the value of the tiles left on his rack, counting 30 points for a joker and face value for all other tiles, and scores this as a minus (negative) amount.

The sum of all the loser's scores is credited to the winner as a plus amount. At the end of the session, each players final score is totalled for the final results. The total of the plus scores should equal the total of the minus scores if all the arithmetic has been done correctly.

In the rare case that all the tiles are used up before anyone goes Rummikub®, the player with the lowest count remaining on his rack is considered the winner. Each of the losers adds up his total remaining tile value, subtracts the winner's total and scores the result as a minus amount, and the winner gets a plus score by totalling the loser's scores.