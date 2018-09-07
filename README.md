# minesweep.rs - Terminal Minesweeper

Toy program as a Rust demo.

## How to Play
`cargo run --release`

Then enter "row col <flag>" to probe cells:

```
Terminal Minesweeper!
Do you feel lucky? Well, do ya, cypherpunk?

            11111
   12345678901234

 1 ..............
 2 ..............
 3 ..............
 4 ..............
 5 ..............
 6 ..............
 7 ..............
 8 ..............
 9 ..............
10 ..............

Enter "row col" to probe, or "row col F" to toggle flag.
> 1 1

            11111
   12345678901234

 1
 2
 3         111
 4 111     1.1
 5 ..1111  1.1
 6 .....1  1.1
 7 .....2112.1
 8 .........21
 9 ....211111
10 ....1

Enter "row col" to probe, or "row col F" to toggle flag.
> 4 10 F

            11111
   12345678901234

 1
 2
 3         111
 4 111     1F1
 5 ..1111  1.1
 6 .....1  1.1
 7 .....2112.1
 8 .........21
 9 ....211111
10 ....1

Enter "row col" to probe, or "row col F" to toggle flag.
> 7 10

!!!BOOOOOOOOOM!!! GAME OVER!

            11111
   12345678901234

 1
 2
 3         111
 4 111     1x1
 5 1x1111  111
 6 1111x1  111
 7 111122112x1
 8 1x212x11x21
 9 112x211111
10   111
```
