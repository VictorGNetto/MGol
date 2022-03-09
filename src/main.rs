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

    let mut parser = Parser::new();
    parser.parse(&mut scanner);

    // loop {
    //     let token = scanner.scan();

    //     let class = token.class;
    //     let lexeme = match token.lexeme {
    //         Some(s) => s,
    //         None => String::from("Nulo"),
    //     };
    //     let tk_type = match token.tk_type {
    //         Some(s) => s,
    //         None => String::from("Nulo"),
    //     };
    //     println!("Classe: {}, Lexema: {}, Tipo: {}", class, lexeme, tk_type);

    //     if class.eq("EOF") {
    //         break;
    //     }
    // }
}
