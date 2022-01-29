use std::fs::File;
use std::io::{BufRead, BufReader};

use super::lexical_automaton::*;
use super::symbol_table::*;
use super::token::*;

pub struct Scanner {
    file: BufReader<File>,
    line: Vec<char>,
    cursor: (usize, usize), // (row, col)
    symbol_table: SymbolTable,
}

impl Scanner {
    pub fn new(file: File) -> Scanner {
        let file = BufReader::new(file);
        let line: Vec<char> = Vec::new();
        let cursor: (usize, usize) = (0, 0);
        let symbol_table = SymbolTable::new();

        Scanner {
            file,
            line,
            cursor,
            symbol_table,
        }
    }

    pub fn scan(&mut self) -> Token {
        let mut lexeme = String::new();
        let mut automaton = Automaton::new();

        while let Some(c) = self.read_char() {
            automaton.advance(c);

            match automaton.action {
                Action::GoBack => self.put_back(),
                Action::Standard => lexeme.push(c),
                Action::ClearLexeme => lexeme.clear(),
                Action::ShowError => self.show_error(c, &automaton.state),
                Action::None => (),
            }

            if automaton.done {
                return self.build_token(lexeme, automaton.state);
            }
        }

        if lexeme.len() > 0 {
            return self.build_token(lexeme, automaton.state);
        }

        Token::new(String::from("EOF"), Some(String::from("EOF")), None)
    }

    // return a char by consuming the internal BufReader file
    fn read_char(&mut self) -> Option<char> {
        static mut EOF_REACHED: bool = false;

        if self.cursor.1 == self.line.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
            let mut s = String::new();
            match self.file.read_line(&mut s) {
                Ok(0) => {
                    // EOF
                    unsafe {
                        EOF_REACHED = true;
                    }
                }
                Ok(_) => self.line = s.chars().collect(),
                Err(_) => (),
            }
        }

        unsafe {
            if EOF_REACHED {
                return None;
            }
        }

        let c = self.line[self.cursor.1];
        self.cursor.1 += 1;

        Some(c)
    }

    // Put the last read character into the 'stream', making it
    // available once again for another read_char()
    fn put_back(&mut self) {
        self.cursor.1 -= 1;
    }

    fn show_error(&self, c: char, automaton_state: &AutomatonState) {
        let line = self.cursor.0;
        let row = self.cursor.1;

        match automaton_state {
            AutomatonState::Error(0) => {
                println!(
                    "Erro léxico na linha {}, coluna {}: '{}' não pertence ao alfabeto",
                    line, row, c
                )
            }
            AutomatonState::Error(1) => {
                println!(
                    "Erro léxico na linha {}, coluna {}: '{}' não inicia nenhum token",
                    line, row, c
                )
            }
            AutomatonState::Error(2) => {
                println!(
                    "Erro léxico na linha {}, coluna {}: após um '.' em um <num> deve vir um dígito, '{}' encontrado",
                    line, row, c
                )
            }
            AutomatonState::Error(3) => {
                println!(
                    "Erro léxico na linha {}, coluna {}: após um 'e' ou 'E' em um <num> deve vir um dígito, um '+' ou um '-', '{}' encontrado",
                    line, row, c
                )
            }
            AutomatonState::Error(4) => {
                println!(
                    "Erro léxico na linha {}, coluna {}: após um 'e+', 'e-', 'E+' ou 'E-' em um <num> deve vir um dígito, '{}' encontrado",
                    line, row, c
                )
            }
            _ => (),
        }
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
            }
            AutomatonState::Accept(2) | AutomatonState::Accept(3) => {
                class = String::from("num");
                tk_type = Some(String::from("real"));
            }
            AutomatonState::Accept(4) => {
                class = String::from("lit");
                tk_type = Some(String::from("literal"));
            }
            AutomatonState::Accept(5) => {
                let lexeme_str = lexeme.clone();
                if let Some(token) = self.symbol_table.get(lexeme_str.unwrap().as_str()) {
                    return token;
                }

                class = String::from("id");
                tk_type = None;
            }
            AutomatonState::Accept(8) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(9) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(10) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(11) => {
                class = String::from("rcb");
                tk_type = None;
            }
            AutomatonState::Accept(12) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(13) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(14) => {
                class = String::from("opr");
                tk_type = None;
            }
            AutomatonState::Accept(15) => {
                class = String::from("opm");
                tk_type = None;
            }
            AutomatonState::Accept(16) => {
                class = String::from("ab_p");
                tk_type = None;
            }
            AutomatonState::Accept(17) => {
                class = String::from("fc_p");
                tk_type = None;
            }
            AutomatonState::Accept(18) => {
                class = String::from("pt_v");
                tk_type = None;
            }
            AutomatonState::Error(_) => {
                class = String::from("ERROR");
                lexeme = None;
                tk_type = None;
            }
            _ => (),
        };

        Token::new(class, lexeme, tk_type)
    }
}
