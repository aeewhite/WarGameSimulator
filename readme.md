# War Card Game Simulator

## Why?

I was curious as to how many turns a game of war typically takes so I built something to simulate many, many iterations of the game

## Specifics

The program uses the [cards](https://crates.io/crates/cards) crate for the playing card simulation. This simulation plays with the following rules:

 - When a player runs out of cards, they lose
 - When a "war" occurs, each player lays 3 cards face down then one more card to challenge the other player
 - If a player runs out of cards during a war, they lose
 - Whoever wins the turn gets all the cards that have been played.

## How to "play"?

`cargo run --release` (release mode definitely makes a big difference) 

Typical output from the program:

```
Left Wins: 253972 (50.7944%), Right Wins: 246028 (49.2056%)
Average # of Turns: 184.00488
```
