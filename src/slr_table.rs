use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum SlrAction {
    S(u8), // shift
    R(u8), // reduce
    Acc,   // accept
    E(u8), // error
}

fn str_to_action(s: &str) -> SlrAction {
    let mut chars = s.chars();
    let kind = chars.next();
    let n = chars.as_str().parse::<u8>().unwrap_or(0);

    match kind {
        Some('S') | Some('s') => SlrAction::S(n),
        Some('R') | Some('r') => SlrAction::R(n),
        Some('A') | Some('a') => SlrAction::Acc,
        Some('E') | Some('e') => SlrAction::E(n),
        _ => SlrAction::E(0),
    }
}

pub struct ActionTable {
    table: HashMap<(u8, String), SlrAction>,
}

impl ActionTable {
    pub fn new() -> ActionTable {
        let terminals = [
            "num",
            "lit",
            "id",
            "opr",
            "rcb",
            "opm",
            "ab_p",
            "fc_p",
            "pt_v",
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
            "$",
        ];

        // open the .csv action table
        let path = "./src/action_table.csv";
        let actions_file = match File::open(path) {
            Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
            Ok(file) => file,
        };

        // create a line iterator over the .csv file
        let mut lines = io::BufReader::new(actions_file).lines();

        // consume the first line that contains the terminals names
        lines.next();

        let mut table = HashMap::new();
        for line in lines {
            if let Ok(okline) = line {
                let actions = okline.split(",").collect::<Vec<&str>>();
                let state = actions[0].parse::<u8>().unwrap();
                // HashMap<(u8, String), SlrAction>
                for i in 0..terminals.len() {
                    let action = actions[i + 1];
                    let terminal = terminals[i];
                    table.insert((state, String::from(terminal)), str_to_action(action));
                }
            }
        }

        ActionTable { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn show(&self) {
        for key in self.table.keys() {
            println!("{:?} -> {:?}", key, self.table.get(key).unwrap());
        }
    }
}

pub struct GotoTable {
    table: HashMap<(u8, String), u8>,
}

impl GotoTable {
    pub fn new() -> GotoTable {
        let nonterminals = [
            "P",
            "V",
            "LV",
            "D",
            "L",
            "TIPO",
            "A",
            "ES",
            "ARG",
            "CMD",
            "LD",
            "OPRD",
            "COND",
            "CAB",
            "EXP_R",
            "CP",
            "R",
            "CABR",
            "CPR",
        ];

        // open the .csv goto table
        let path = "./src/goto_table.csv";
        let actions_file = match File::open(path) {
            Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
            Ok(file) => file,
        };

        // create a line iterator over the .csv file
        let mut lines = io::BufReader::new(actions_file).lines();

        // consume the first line that contains the nonterminals names
        lines.next();

        let mut table = HashMap::new();
        for line in lines {
            if let Ok(okline) = line {
                let gotos = okline.split(",").collect::<Vec<&str>>();
                let state = gotos[0].parse::<u8>().unwrap();
                // HashMap<(u8, String), u8>
                for i in 0..nonterminals.len() {
                    let goto = gotos[i + 1].parse::<u8>().unwrap();
                    if goto == 0 {
                        continue;
                    }
                    let nonterminal = nonterminals[i];
                    table.insert((state, String::from(nonterminal)), goto);
                }
            }
        }

        GotoTable { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn show(&self) {
        for key in self.table.keys() {
            println!("{:?} -> {}", key, self.table.get(key).unwrap());
        }
    }
}
