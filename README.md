# Algorithm Implementations in Rust

## Overview

This repository contains Rust implementations of classic computer science algorithms, providing efficient and well-documented solutions to fundamental problems.

## Included Algorithms

### 1. N-Queens Solver
Solves the N-Queens problem using backtracking to find all possible arrangements of N queens on an N×N chessboard.

**Features:**
- Finds all valid solutions
- Interactive solution navigation
- Visual board representation

### 2. Huffman Coding
Implements the Huffman compression algorithm for lossless text encoding.

**Features:**
- Character frequency analysis
- Optimal prefix-free code generation
- Binary encoding output

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

Clone the repository:
```bash
git clone https://github.com/naseridev/algorithm-design.git
```

```bash
cd algorithm-design
```

### Building

Build all projects:
```bash
cargo build --release
```

### Running

#### N-Queens Solver
```bash
cd n-queens-solver
cargo run --release
```

Follow the on-screen prompts to navigate through solutions.

#### Huffman Coding
```bash
cd huffman-coding
cargo run --release
```

Enter text when prompted to see the Huffman encoding.

## Dependencies

### N-Queens Solver
- clearscreen: Terminal clearing utility

### Huffman Coding
- No external dependencies (uses std library only)

## Documentation

Each algorithm has its own detailed README in its respective directory:
- [N-Queens Solver Documentation](n-queens-solver/README.md)
- [Huffman Coding Documentation](huffman-coding/README.md)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.
