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

    while let Some(token) = scanner.scan() {
        println!("{}", token.c);
    }
}
