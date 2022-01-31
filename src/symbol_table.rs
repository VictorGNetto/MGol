use std::collections::HashMap;

use super::token::*;

// A struct to represent a Symbol Table. It just
// a wrapper of a HashMap<String, Token>.
pub struct SymbolTable {
    hashmap: HashMap<String, Token>,
}

impl SymbolTable {
    // create a new SymbolTable (already with the MGol reserved words)
    pub fn new() -> SymbolTable {
        let hashmap = HashMap::new();

        let mut symbol_table = SymbolTable { hashmap };
        symbol_table.init_reserved_words();

        symbol_table
    }

    // get a Token from the Symbol Table
    pub fn get(&self, lexeme: String) -> Option<Token> {
        if let Some(token) = self.hashmap.get(lexeme.as_str()) {
            return Some(Token::new_from_ref(token));
        }

        None
    }

    // insert a Token into the Symbol Table
    pub fn insert(&mut self, lexeme: String, token: Token) {
        self.hashmap.insert(lexeme, token);
    }

    // update a Token from the Symbol Table
    pub fn update(&mut self, lexeme: String, token: Token) {
        if self.hashmap.contains_key(lexeme.as_str()) {
            self.insert(lexeme, token);
        }
    }

    // put all the MGol reserved words into the Symbol Table
    fn init_reserved_words(&mut self) {
        let reserved_words = [
            "inicio",
            "varinicio",
            "varfim",
            "escreva",
            "leia",
            "se",
            "entao",
            "fimse",
            "repita",
            "fimrepita",
            "fim",
            "inteiro",
            "literal",
            "real",
        ];

        for lexeme in reserved_words {
            self.hashmap
                .insert(String::from(lexeme), Token::new_from_lexeme(lexeme));
        }
    }
}
