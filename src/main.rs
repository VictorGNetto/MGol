use mgol::scanner::Scanner;
use std::fs::File;

use mgol::grammar::Grammar;
use mgol::slr_table::ActionTable;

fn main() {
    // open the file
    let path = "./test/teste.mgol";
    let file = match File::open(path) {
        Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
        Ok(file) => file,
    };

    // start the scanner
    let mut scanner = Scanner::new(file);

    loop {
        let token = scanner.scan();

        let class = token.class;
        let lexeme = match token.lexeme {
            Some(s) => s,
            None => String::from("Nulo"),
        };
        let tk_type = match token.tk_type {
            Some(s) => s,
            None => String::from("Nulo"),
        };
        println!("Classe: {}, Lexema: {}, Tipo: {}", class, lexeme, tk_type);

        if class.eq("EOF") {
            // println!("----------------");
            // scanner.show_symbol_table();
            break;
        }
    }

    let grammar = Grammar::new();
    grammar.show();

    let action_table = ActionTable::new();
    // action_table.show();
    println!("{}", action_table.len());
}
