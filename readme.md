## What is this?
This is a program to generate solutions for a sudoku ruleset i'm experimenting with.

## Usage

### CLI

```bash
# Solve using default settings (engine v2, standard rules, sudoku.txt)
cargo run

# Use extended sum-sequence rules
cargo run -- --sum-sequence

# Specify a custom sudoku file
cargo run -- --file path/to/puzzle.txt

# Override iteration limit (default: 2000000)
cargo run -- --limit 5000000

# Use engine v1 (cage pair combinations generator)
cargo run -- --engine 1

# Show help
cargo run -- --help
```

### Input Format

Sudoku files should contain 9 rows of 9 numbers (0 for empty cells), separated by spaces or commas:

```
0 6 0 8 0 0 0 0 0
4 0 0 0 0 5 0 8 0
0 3 7 0 0 0 0 0 0
...
```

## Sum Sequence Puzzle Board setup

https://f-puzzles.com/?id=23hb5lph

### Palindromes
```
 _________________
|     |     |     |
|     |  1  |2    |
|_____|1___2|_____|
|    1|  3  |4    |
|  1  |3   4|  6  |
|____3|__5__|6____|
|  3  |5   6|     |
|     |  6  |     |
|_____|_____|_____|
```

### Cages
```
 _________________
|    0|  0  |0    |
|    0|  0  |0    |
|1_1__|_____|__1_1|
|     |     |     |
|1 1  |     |  1 1|
|_____|_____|_____|
|1 1  |     |  1 1|
|    0|  0  |0    |
|____0|__0__|0____|
```

### Set deduction
```
 _________________
|    0|  0  |0    |
|    0|     |     |
|1_1__|_____|__1_1|
|     |     |     |
|1    |     |    1|
|_____|_____|_____|
|1    |     |  1 1|
|    0|     |0    |
|____0|__0__|0____|
```

Todo:
- [ ] Find where to put the 3 missing triplet cages on the corner boxes
- [ ] Force palindrome deduction (Digit inside the cage from palindrome 3 must be the same as the one from palindrome 2)

## Rules

- All cage sums are different.
- Cage sums form a continuous sequence.
- Every cage digit not in a palindrome appears exactly three times.

## Intended Logic

### Structure
- 12 pair cages
- 3 triplet cages
- 6 cells in pair cages are part of palindromes

### Deduction (not present in the current board setup)
- Palindrome cells consist of 3 distinct digits, each appearing twice.

## Mathematical Deduction

### 1. Baseline sum
All cage digits not in palindromes appear exactly three times.

1 + 2 + ... + 9 = 45  
3 × 45 = 135  

Total of all cage sums = 135 + X  
where X is the contribution of the palindrome cells.

### 2. Palindrome contribution
Each palindrome digit appears twice:

X = 2(a + b + c)

Minimum: 2(1+2+3) = 12  
Maximum: 2(7+8+9) = 48  

So: 12 ≤ X ≤ 48

### 3. Consecutive cage sums
There are 15 cages with consecutive sums starting at S:

S + (S+1) + ... + (S+14)  
= 15S + (0 + 1 + ... + 14)  
= 15S + 105

### 4. Equating totals
15S + 105 = 135 + X  
15S = 30 + X  
S = 2 + X / 15

### 5. Divisibility constraint
S must be an integer.

X must be divisible by 15 and even.  
With 12 ≤ X ≤ 48, the only solution is:

X = 30

### 6. Result
S = 4  
a + b + c = 15

### 7. Valid palindrome digit sets
Digits 1–9, no repetition, sum to 15:

159, 168, 249, 258, 267, 348, 357, 456

### 8. Forced cage sum sequence
4 5 6 7 8 9 10 11 12 13 14 15 16 17 18
