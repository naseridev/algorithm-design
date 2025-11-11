use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Write};

struct Node {
    character: Option<char>,
    frequency: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

pub struct HuffmanCoding;

impl Default for HuffmanCoding {
    fn default() -> Self {
        Self::new()
    }
}

impl HuffmanCoding {
    pub fn new() -> Self {
        HuffmanCoding
    }

    pub fn encode(&self, text: &str) -> HashMap<char, String> {
        if text.is_empty() {
            return HashMap::new();
        }

        let freq = self.build_frequency_table(text);

        let mut queue = BinaryHeap::new();
        for (character, frequency) in freq {
            queue.push(Node {
                character: Some(character),
                frequency,
                left: None,
                right: None,
            });
        }

        while queue.len() > 1 {
            let left = queue.pop().unwrap();
            let right = queue.pop().unwrap();

            let merged = Node {
                character: None,
                frequency: left.frequency + right.frequency,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            queue.push(merged);
        }

        let root = queue.pop().unwrap();
        let mut codes = HashMap::new();
        Self::generate_codes_helper(&root, String::new(), &mut codes);

        codes
    }

    fn build_frequency_table(&self, text: &str) -> HashMap<char, i32> {
        let mut freq = HashMap::new();

        for c in text.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        freq
    }

    fn generate_codes_helper(node: &Node, code: String, codes: &mut HashMap<char, String>) {
        if node.is_leaf() {
            if let Some(character) = node.character {
                codes.insert(character, code);
            }
            return;
        }

        if let Some(ref left) = node.left {
            Self::generate_codes_helper(left, code.clone() + "0", codes);
        }

        if let Some(ref right) = node.right {
            Self::generate_codes_helper(right, code + "1", codes);
        }
    }
}

fn main() {
    println!("Enter text to encode:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim();

    let huffman = HuffmanCoding::new();
    let codes = huffman.encode(text);

    println!("\nHuffman Codes:");
    for (character, code) in &codes {
        println!("'{}': {}", character, code);
    }

    println!("\nEncoded message:");
    for c in text.chars() {
        if let Some(code) = codes.get(&c) {
            print!("{}", code);
        }
    }
    println!();
}
