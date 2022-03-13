use std::fs::File;
use mgol::scanner::Scanner;
use mgol::parser::Parser;

fn main() {
    // open the file
    let path = "./test/teste.mgol";
    let file = match File::open(path) {
        Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
        Ok(file) => file,
    };

    // start the scanner
    let mut scanner = Scanner::new(file);

    // start the parser and give it the scanner to begin the syntactic analysis
    let mut parser = Parser::new();
    parser.parse(&mut scanner);
}
