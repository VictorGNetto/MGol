// A struct to represent a Token: class is just a
// String, while lexeme and tk_type is a Option<String>.
// When lexeme or tk_type is None, means that the Token
// doesn't have those attributes.
#[derive(Clone)]
pub struct Token {
    pub class: String,
    pub lexeme: Option<String>,
    pub tk_type: Option<String>,
}

impl Token {
    // create a new Token from a class, lexeme and tk_type
    pub fn new(class: String, lexeme: Option<String>, tk_type: Option<String>) -> Token {
        Token {
            class,
            lexeme,
            tk_type,
        }
    }

    // create a new Token given a lexeme
    pub fn new_from_lexeme(lexeme: &str) -> Token {
        Token {
            class: String::from(lexeme),
            lexeme: Some(String::from(lexeme)),
            tk_type: Some(String::from(lexeme)),
        }
    }

    // create a new Token from another Token reference
    pub fn new_from_ref(token: &Token) -> Token {
        token.clone()
    }
}
