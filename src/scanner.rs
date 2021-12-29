use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Token;

pub struct Scanner {
    file: BufReader<File>,
    line: Vec<char>,
    cursor: usize,
}

impl Scanner {
    pub fn new(file: File) -> Scanner {
        let file = BufReader::new(file);
        let line: Vec<char> = Vec::new();
        let cursor: usize = 0;

        Scanner { file, line, cursor }
    }

    pub fn scan(&mut self) -> Option<Token> {
        if let Some(c) = self.read_char() {
            return Some(Token { c });
        }

        None
    }

    fn read_char(&mut self) -> Option<char> {
        if self.cursor == self.line.len() {
            self.cursor = 0;
            let mut s = String::new();
            match self.file.read_line(&mut s) {
                Ok(0) => return None, // EOF
                Ok(_) => self.line = s.chars().collect(),
                _ => (),
            }
        }

        let c = self.line[self.cursor];
        self.cursor += 1;

        Some(c)
    }
}