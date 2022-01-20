pub struct Token {
    pub class: String,
    pub lexeme: Option<String>,
    pub tk_type: Option<String>,
}

impl Token {
    pub fn new(class: String, lexeme: Option<String>, tk_type: Option<String>) -> Token {
        Token {
            class,
            lexeme,
            tk_type,
        }
    }

    pub fn new_from_lexeme(lexeme: &str) -> Token {
        Token {
            class: String::from(lexeme),
            lexeme: Some(String::from(lexeme)),
            tk_type: Some(String::from(lexeme)),
        }
    }
}
