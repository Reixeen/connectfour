Connect Four is a 7-wide and 6-tall boardspace

2 players take turns placing a token into a column, the token falls to the lowest available slot
The game continues until a player has aligned 4 of their tokens in any cardinal or ordinal direction
The order in which a winning alignment is placed does not matter

A 7 wide board does not fit into sqrt(64) such as with chess' 8-wide board

We use the standard chess bitboard implementation, but ignore the final column and top 2 rows, instead of re-factoring chess engine documentation for 7-wide boards

Chess board, for reference
8|56,57,58,59,60,61,62,63|\n
7|48,49,50,51,52,53,54,55|\n
6|40,41,42,43,44,45,46,47|\n
5|32,33,34,35,36,36,38,39|\n
4|24,25,26,27,28,29,30,31|\n
3|16,17,18,19,20,21,22,23|\n
2|8 ,9 ,10,11,12,13,14,15|\n
1|0 ,1 ,2 ,3 ,4 ,5 ,6 ,7 |\n
  A ,B ,C ,D ,E ,F ,G ,H

ConnectFourBoard, with the chess bit values, for reference
|40,41,42,43,44,45,46|
|32,33,34,35,36,36,38|
|24,25,26,27,28,29,30|
|16,17,18,19,20,21,22|
|8 ,9 ,10,11,12,13,14|
|0 ,1 ,2 ,3 ,4 ,5 ,6 |
----------------------
|A ,B ,C ,D ,E ,F ,G |

2 u64 bitboards will be used to store the entire board - 1 per player

<< 8 moves up a row
<< 1 moves one column to the right

### Testing completed endgames
Spaces 47 -> 64 must always be empty and must not be used in verification
There are 4 directions in which boards need to be verified:
 | Vertical:          [x], [x-7], [x-14], [x-21] => - 7 === 0x1010101 ending at the bottom left
 - Horizontal:        [x], [x+1], [x+2],  [x+3]  => + 1 === 0xF
 / Forward diagonal:  [x], [x+8], [x+16], [x+24] => + 8 === 0x8040201
 \ Backward diagonal: [x], [x+6], [x+12], [x+16] => + 6 === 0x1020408

We also need to ensure verification is only done with consecutive columns - this game has solid walls at each end and does not wrap
This can be done in a few ways:
1. runtime checks that the current adjudicated space is not wrapped from the previous
2. have specific starting points for the mask of each verification type
3. create a mask for all possible wins, check them 1 by 1

Huge thank you to the following:
https://nnajm.github.io/visual-bb/ for easy hex conversions of applicable bits
https://rustic-chess.org/ for bitboard explanations
https://github.com/cosmobobak for an excellent, inspiring, RustWeek talk about Viri:
https://www.youtube.com/watch?v=Gx21yYLwn10
https://github.com/cosmobobak/viridithas 