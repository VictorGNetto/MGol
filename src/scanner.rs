use std::fs::File;
use std::io::{BufRead, BufReader};

use super::lexical_automaton::*;
use super::token::*;

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
                Action::Standard => lexeme.push(c),
                Action::ClearLexeme => lexeme.clear(),
                Action::ShowError => {
                    println!("Error sintático na linha 4, coluna {}: {}", self.cursor, c);
                }
                Action::None => (),
            }

            if automaton.done {
                return self.build_token(lexeme, automaton.state);
            }
        }

        Token::new(String::from("EOF"), Some(String::from("EOF")), None)
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

        let mut class = String::new();
        let mut lexeme = Some(lexeme);
        let mut tk_type = Some(String::new());

        match automaton_state {
            AutomatonState::Accept(1) => {
                class = String::from("num");
                tk_type = Some(String::from("inteiro"));
            },
            AutomatonState::Accept(2) | AutomatonState::Accept(3) => {
                class = String::from("num");
                tk_type = Some(String::from("real"));
            },
            AutomatonState::Accept(4) => {
                class = String::from("lit");
                tk_type = Some(String::from("literal"));
            },
            AutomatonState::Accept(5) => {
                class = String::from("id");
                tk_type = None;
            },
            AutomatonState::Accept(8) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(9) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(10) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(11) => {
                class = String::from("rcb");
                tk_type = None;
            },
            AutomatonState::Accept(12) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(13) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(14) => {
                class = String::from("opr");
                tk_type = None;
            },
            AutomatonState::Accept(15) => {
                class = String::from("opm");
                tk_type = None;
            },
            AutomatonState::Accept(16) => {
                class = String::from("ab_p");
                tk_type = None;
            },
            AutomatonState::Accept(17) => {
                class = String::from("fc_p");
                tk_type = None;
            },
            AutomatonState::Accept(18) => {
                class = String::from("pt_v");
                tk_type = None;
            },
            AutomatonState::Error => {
                class = String::from("ERROR");
                lexeme = None;
                tk_type = None;
            },
            _ => ()
        };

        Token::new(class, lexeme, tk_type)
    }
}
