use std::collections::HashMap;

use super::token::*;

pub struct SymbolTable {
    hashmap: HashMap<String, Token>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let hashmap = HashMap::new();

        let mut symbol_table = SymbolTable { hashmap };
        symbol_table.init_reserved_words();

        symbol_table
    }

    pub fn get(&self, lexeme: &str) -> Option<Token> {
        if let Some(token) = self.hashmap.get(lexeme) {
            return Some(Token::new_from_ref(token));
        }

        None
    }

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
