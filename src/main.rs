use std::fs::File;
use mgol::scanner::Scanner;

fn main() {
    // Abre o arquivo
    let path = "./test/teste.mgol";
    let file = match File::open(path) {
        Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
        Ok(file) => file,
    };

    // Inicializa o Scanner
    let mut scanner = Scanner::new(file);

    loop {
        let token = scanner.scan();

        let class = token.class;
        let lexeme = token.lexeme;
        let tk_type = match token.tk_type {
            Some(s) => s,
            None => String::from("Nulo"),
        };
        println!("Classe: {}, Lexema: {}, Tipo: {}", class, lexeme, tk_type);

        if class.eq("EOF") {
            break;
        }
    }
}
