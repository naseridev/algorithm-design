use std::io::{self, Write};

pub struct NQueensSolver {
    solutions: Vec<Vec<Vec<u8>>>,
    current_solution_index: usize,
}

impl Default for NQueensSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl NQueensSolver {
    pub fn new() -> Self {
        NQueensSolver {
            solutions: Vec::new(),
            current_solution_index: 0,
        }
    }

    pub fn solve(&mut self, n: usize) {
        self.solutions.clear();
        self.current_solution_index = 0;

        let mut board = vec![vec![0u8; n]; n];
        self.place_queens(&mut board, 0, n);

        if self.solutions.is_empty() {
            println!("There is no solution for this value of N.");
            return;
        }

        self.show_solution(self.current_solution_index);

        loop {
            println!("\nEnter command (n=next, p=previous, q=quit):");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "n" => {
                    self.current_solution_index =
                        (self.current_solution_index + 1) % self.solutions.len();
                    self.show_solution(self.current_solution_index);
                }
                "p" => {
                    self.current_solution_index =
                        (self.current_solution_index + self.solutions.len() - 1)
                            % self.solutions.len();
                    self.show_solution(self.current_solution_index);
                }
                "q" => break,
                _ => println!("Invalid command"),
            }
        }
    }

    fn place_queens(&mut self, board: &mut Vec<Vec<u8>>, col: usize, n: usize) -> bool {
        if col == n {
            let solution = board.clone();
            self.solutions.push(solution);
            return false;
        }

        let mut found_any = false;

        for row in 0..n {
            if self.is_safe(board, row, col, n) {
                board[row][col] = 1;
                found_any |= self.place_queens(board, col + 1, n);
                board[row][col] = 0;
            }
        }

        found_any
    }

    fn is_safe(&self, board: &[Vec<u8>], row: usize, col: usize, n: usize) -> bool {
        for i in 0..col {
            if board[row][i] == 1 {
                return false;
            }
        }

        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 1 {
                return false;
            }
            i -= 1;
            j -= 1;
        }

        let mut i = row + 1;
        let mut j = col as i32 - 1;
        while i < n && j >= 0 {
            if board[i][j as usize] == 1 {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }

    fn show_solution(&self, index: usize) {
        clearscreen::clear().expect("failed to clear screen");

        let board = &self.solutions[index];
        let n = board.len();

        println!(
            "Solution number {} of {} for problem {}-Minister:",
            index + 1,
            self.solutions.len(),
            n
        );

        for row in board.iter().take(n) {
            for cell in row.iter().take(n) {
                if *cell == 1 {
                    print!("Q ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut solver = NQueensSolver::new();
    solver.solve(8);
}
