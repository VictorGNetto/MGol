use std::fs::File;
use std::io::{BufRead, BufReader};

use super::lexical_automaton::*;
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

    pub fn scan(&mut self) -> Token {
        let mut lexeme = String::new();
        let mut automaton = Automaton::new();

        while let Some(c) = self.read_char() {
            automaton.advance(c);

            if automaton.go_back {
                self.put_back();
            } else {
                lexeme.push(c);
            }

            if automaton.done {
                return self.build_token(lexeme);
            }
        }

        return Token {
            class: String::from("EOF"),
            lexeme: String::from("EOF"),
            tk_type: None,
        };
    }

    // return a char by consuming the internal BufReader file
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

    // Put the last read character into the 'stream', making it
    // available once again for another read_char()
    fn put_back(&mut self) {
        self.cursor -= 1;
    }

    // A Token fabric
    fn build_token(&mut self, lexeme: String) -> Token {
        return Token {
            class: String::from("..."),
            lexeme: lexeme,
            tk_type: None,
        };
    }
}
