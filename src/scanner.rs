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
            // println!("{:?}", automaton.state);

            match automaton.action {
                Action::GoBack => self.put_back(),
                Action::UpdateLexeme => lexeme.push(c),
                Action::ClearLexeme => lexeme.clear(),
                Action::ShowError => {
                    println!("Error sintático na linha 4, coluna {}: {}", self.cursor, c);
                },
                Action::None => (),
            }

            if automaton.done {
                return self.build_token(lexeme, automaton.state);
            }
        }

        Token {
            class: String::from("EOF"),
            lexeme: String::from("EOF"),
            tk_type: None,
        }
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
    fn build_token(&mut self, lexeme: String, automaton_state: AutomatonState) -> Token {
        // println!(">>> {:?}", automaton_state);

        match automaton_state {
            AutomatonState::Accept(1) => Token {
                class: String::from("Num"),
                lexeme,
                tk_type: Some(String::from("inteiro")),
            },
            AutomatonState::Accept(2) | AutomatonState::Accept(3) => Token {
                class: String::from("Num"),
                lexeme,
                tk_type: Some(String::from("real")),
            },
            AutomatonState::Accept(4) => Token {
                class: String::from("Lit"),
                lexeme,
                tk_type: Some(String::from("literal")),
            },
            AutomatonState::Accept(5) => Token {
                class: String::from("id"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(8) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(9) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(10) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(11) => Token {
                class: String::from("RCB"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(12) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(13) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(14) => Token {
                class: String::from("OPR"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(15) => Token {
                class: String::from("OPM"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(16) => Token {
                class: String::from("AB_P"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(17) => Token {
                class: String::from("FC_P"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Accept(18) => Token {
                class: String::from("PT_V"),
                lexeme,
                tk_type: None,
            },
            AutomatonState::Error => Token {
                class: String::from("ERROR"),
                lexeme: String::from("NULO"),
                tk_type: None,
            },
            _ => Token {
                class: String::from(""),
                lexeme: lexeme,
                tk_type: None,
            },
        }
    }
}
