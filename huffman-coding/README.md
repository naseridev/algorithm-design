# Huffman Coding

## Overview

Huffman coding is a lossless data compression algorithm that uses variable-length codes to represent characters based on their frequency of occurrence. Characters that appear more frequently are assigned shorter codes, resulting in efficient compression.

## Algorithm Description

Huffman coding is a **greedy algorithm** that builds an optimal prefix-free binary code tree.

### Key Concepts

- **Prefix-free codes**: No code is a prefix of another, enabling unambiguous decoding
- **Variable-length encoding**: Frequent characters get shorter codes
- **Binary tree representation**: Internal nodes represent frequency sums, leaves represent characters

## Algorithm Steps

1. **Frequency Analysis**: Count the occurrence of each character in the input text
2. **Priority Queue Construction**: Create a min-heap with nodes for each character
3. **Tree Building**:
   - Extract two nodes with minimum frequency
   - Create a parent node with combined frequency
   - Insert the parent back into the priority queue
   - Repeat until one node remains (the root)
4. **Code Generation**: Traverse the tree to assign binary codes
   - Left edge = 0
   - Right edge = 1

### Time Complexity

- Frequency table construction: O(n) where n is text length
- Tree building: O(m log m) where m is number of unique characters
- Code generation: O(m)
- Overall: O(n + m log m)

### Space Complexity

- O(m) for frequency table and code storage
- O(m) for the Huffman tree

## Implementation Details

### Data Structures

- `Node`: Represents tree nodes with character, frequency, and child pointers
- `BinaryHeap`: Min-heap for efficient extraction of minimum frequency nodes
- `HashMap`: Stores character-to-code mappings

### Core Functions

- `build_frequency_table()`: Analyzes input text and counts character occurrences
- `encode()`: Main function that orchestrates the encoding process
- `generate_codes_helper()`: Recursively traverses the tree to generate binary codes

### Tree Construction

The algorithm builds a bottom-up tree by:
1. Starting with leaf nodes for each character
2. Repeatedly merging the two lowest-frequency nodes
3. Creating a parent node with combined frequency
4. Continuing until a single root node remains

## Usage

The program reads text input and generates Huffman codes for each character.

### Input

Enter any text string when prompted.

### Output

- Character-to-code mapping table
- Encoded binary representation of the input text

## Example

### Input
```
hello
```

### Output
```
Huffman Codes:
'h': 00
'e': 01
'l': 1
'o': 10

Encoded message:
0001111010
```

## Compression Ratio

The compression ratio depends on:
- Character frequency distribution
- Number of unique characters
- Length of input text

Texts with skewed frequency distributions achieve better compression.

## Building and Running

```bash
cargo build --release
cargo run --release
```

## Applications

- File compression (ZIP, GZIP)
- Image compression (JPEG)
- Network protocols
- Database systems

## References

- Huffman, D. A. (1952). A Method for the Construction of Minimum-Redundancy Codes
- Cormen, T. H., et al. (2009). Introduction to Algorithms, Third Edition