# Sudoku solver in Rust

## Intro
My very first personal Rust project. The software can solve sudoku puzzles based on a .sdm input file and print the solutions to the terminal in a Simple Sudoku format.

I learned a lot about implementing OOP and functional concepts in Rust, and of course I learned a tremendous amount about memory and the borrow checker.

## Usage
The source code can be run with **cargo**. It is important to tell the program the path of the input file. There is an example file in the /assets directory. As it is brute force, it might take a couple of seconds for it to solve all the puzzles.
```bash
cargo run -- -f ./assets/example.sdm
```
##### Screenshot
![Screenshot of the program running in the terminal](https://i.imgur.com/uxbX52I.png)

## Features
#### Current
- Read sudoku puzzles based on .sdm files (containing one or multiple puzzles).
- Solve the puzzles using a brute force method.
- Convert the input and the result puzzles into human-readable Simple Sudoku format.
#### Planned
- More sophisticated solver algorithms.
- Ability to write the solutions into .ss files.

## Formats
#### .sdm

Format where each line represents a sudoku puzzle. 0 represents an empty space. Can have multiple lines.
```
016400000200009000400000062070230100100000003003087040960000005000800007000006820
049008605003007000000000030000400800060815020001009000010000000000600400804500390
760500000000060008000000403200400800080000030005001007809000000600010000000003041
```

#### .ss/Simple Sudoku

Human-readable sudoku format.
```
1..|...|7..
.2.|...|5..
6..|38.|...
-----------
.78|...|...
...|6.9|...
...|...|14.
-----------
...|.25|..9
..3|...|.6.
..4|...|..2
```
