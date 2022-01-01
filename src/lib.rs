pub mod scanner;
pub mod lexical_automaton;

pub struct Token {
    pub class: String,
    pub lexeme: String,
    pub tk_type: Option<String>,
}
