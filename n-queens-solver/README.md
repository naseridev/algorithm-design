# N-Queens Solver

## Overview

The N-Queens problem is a classic combinatorial optimization problem that involves placing N chess queens on an N×N chessboard such that no two queens threaten each other. This implementation provides a complete solution using backtracking algorithm.

## Problem Description

The challenge is to place N queens on an N×N board where:
- No two queens share the same row
- No two queens share the same column
- No two queens share the same diagonal

## Algorithm

This implementation uses a **backtracking algorithm** with the following approach:

1. **Initialization**: Start with an empty N×N board
2. **Recursive Placement**: Place queens column by column from left to right
3. **Safety Check**: Before placing a queen, verify it doesn't conflict with previously placed queens
4. **Backtracking**: If placement fails, remove the queen and try the next position
5. **Solution Collection**: Store all valid board configurations

### Time Complexity

- Worst case: O(N!)
- The algorithm explores all possible placements and prunes invalid branches

### Space Complexity

- O(N²) for board representation
- O(N) for recursion depth

## Implementation Details

### Core Functions

- `place_queens()`: Recursive function that attempts to place queens column by column
- `is_safe()`: Validates whether a queen can be placed at a given position
- `show_solution()`: Displays the current solution in a visual format

### Safety Checking

The `is_safe()` function checks three conditions:
1. **Row check**: Ensures no queen exists in the same row to the left
2. **Upper diagonal check**: Validates the upper-left diagonal
3. **Lower diagonal check**: Validates the lower-left diagonal

## Usage

The solver finds all possible solutions for a given N value and allows interactive navigation through solutions.

### Commands

- `n`: Display next solution
- `p`: Display previous solution
- `q`: Quit the program

## Example Output

```
Solution number 1 of 92 for problem 8-Minister:
Q . . . . . . .
. . . . Q . . .
. . . . . . . Q
. . . . . Q . .
. . Q . . . . .
. . . . . . Q .
. Q . . . . . .
. . . Q . . . .
```

## Technical Notes

- The implementation stores all solutions in memory for easy navigation
- Board state uses a 2D vector representation (0 for empty, 1 for queen)
- The solver generates all solutions before displaying the first one

## Building and Running

```bash
cargo build --release
cargo run --release
```

## Dependencies

- clearscreen: For clearing terminal output

## References

- Wirth, N. (1976). Algorithms + Data Structures = Programs
- Knuth, D. E. (1975). The Art of Computer Programming, Volume 4