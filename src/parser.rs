use super::grammar::Grammar;
use super::obj_file::ObjFile;
use super::scanner::Scanner;
use super::slr_table::{ActionTable, GotoTable, SlrAction};
use super::symbol_table::SymbolTable;
use super::token::Token;

struct SyntaticStack {
    stack: Vec<u8>,
}

impl SyntaticStack {
    fn new() -> SyntaticStack {
        SyntaticStack { stack: vec![0] }
    }

    fn top(&self) -> u8 {
        self.stack[self.stack.len() - 1]
    }

    fn push(&mut self, n: u8) {
        self.stack.push(n);
    }

    fn pop(&mut self, count: u8) {
        for _ in 0..count {
            self.stack.pop();
        }
    }
}

struct SemanticStack {
    stack: Vec<(String, String, String)>, // Vec<(Item, lexeme, tk+_ype)>
}

impl SemanticStack {
    fn new() -> SemanticStack {
        SemanticStack { stack: Vec::new() }
    }

    fn top(&self) -> &(String, String, String) {
        &self.stack[self.stack.len() - 1]
    }

    fn push(&mut self, attrs: (String, String, String)) {
        self.stack.push(attrs);
    }

    fn pop(&mut self, count: u8) {
        for _ in 0..count {
            self.stack.pop();
        }
    }
}

pub struct Parser {
    syntatic_stack: SyntaticStack,
    grammar: Grammar,
    action_table: ActionTable,
    goto_table: GotoTable,
    token_buffer: Vec<Token>,
    syntatic_error_msgs: Vec<String>,
    semantic_error_msgs: Vec<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            syntatic_stack: SyntaticStack::new(),
            grammar: Grammar::new(),
            action_table: ActionTable::new(),
            goto_table: GotoTable::new(),
            token_buffer: Vec::new(),
            syntatic_error_msgs: Vec::new(),
            semantic_error_msgs: Vec::new(),
        }
    }

    pub fn parse(&mut self, scanner: &mut Scanner) {
        let mut obj_file = ObjFile::new();
        let mut semantic_stack = SemanticStack::new();

        let mut token = self.next_token(scanner);
        let mut a = token.class.clone();
        loop {
            let s = self.syntatic_stack.top();
            let action = self.action_table.get(&(s, a.clone()));
            match action {
                SlrAction::S(t) => {
                    self.syntatic_stack.push(t);
                    token = self.next_token(scanner);
                    a = token.class.clone();
                }
                SlrAction::R(r) => {
                    let rule = self.grammar.get_rule(r as usize);
                    rule.show();
                    self.run_semantic_rule(
                        r,
                        &mut obj_file,
                        &mut semantic_stack,
                        &mut scanner.symbol_table,
                    );
                    #[allow(non_snake_case)]
                    let A = rule.left;
                    let beta = rule.right;
                    self.syntatic_stack.pop(beta.len() as u8);
                    let t = self.syntatic_stack.top();
                    self.syntatic_stack
                        .push(self.goto_table.get(&(t, A.text.clone())));
                }
                SlrAction::Acc => break,
                SlrAction::E(e) => {
                    // put the last read Token back into the input
                    self.token_buffer.push(token);

                    // call the error recovery procedure
                    // if it can't solve the problem, stop the analysis
                    if !self.error_recovery(e, scanner) {
                        break;
                    }

                    // read the next Token, since the error recoery may change the input
                    token = self.next_token(scanner);
                    a = token.class.clone();
                }
            }
        }

        let lexical_errors = scanner.show_lexical_error_msgs();
        let syntatic_errors = self.show_syntatic_error_msgs();
        let semantic_errors = self.show_semantic_error_msgs();

        if lexical_errors + syntatic_errors + semantic_errors == 0 {
            obj_file.create();
        }
    }

    fn next_token(&mut self, scanner: &mut Scanner) -> Token {
        if self.token_buffer.is_empty() {
            return scanner.safe_scan();
        } else {
            return self.token_buffer.pop().unwrap();
        }
    }

    fn run_semantic_rule(
        &self,
        r: u8,
        obj_file: &mut ObjFile,
        semantic_stack: &mut SemanticStack,
        symbol_table: &mut SymbolTable,
    ) {
        match r {
            8 => {
                let token = symbol_table.get(String::from("inteiro")).unwrap();
                semantic_stack.push((
                    String::from("TIPO"),
                    String::from(""),
                    token.tk_type.unwrap(),
                ));
                let (_, _, tk_type) = semantic_stack.top();
                obj_file.print(tk_type.clone());
            }
            9 => {
                let token = symbol_table.get(String::from("real")).unwrap();
                semantic_stack.push((
                    String::from("TIPO"),
                    String::from(""),
                    token.tk_type.unwrap(),
                ));
                let (_, _, tk_type) = semantic_stack.top();
                obj_file.print(tk_type.clone());
            }
            10 => {
                let token = symbol_table.get(String::from("literal")).unwrap();
                semantic_stack.push((
                    String::from("TIPO"),
                    String::from(""),
                    token.tk_type.unwrap(),
                ));
                let (_, _, tk_type) = semantic_stack.top();
                obj_file.print(tk_type.clone());
            }
            _ => (),
        }
    }

    fn error_recovery(&mut self, error_code: u8, scanner: &mut Scanner) -> bool {
        // Some syntatic errors may be recovered and some may not.
        // For those who can not be recovered, sometimes a infinite loop
        // takes place and the syntatic analysis never ends. To prevent this,
        // we limit the maximum number of errors recovery to be 100.
        const MAX_SIYNTATIC_ERRORS: u8 = 100;
        static mut SYNTATIC_ERRORS: u8 = 0;
        unsafe {
            SYNTATIC_ERRORS += 1;
            if SYNTATIC_ERRORS > MAX_SIYNTATIC_ERRORS {
                return false;
            }
        }

        match error_code {
            // code found after the 'fim'  keyword
            1 => {
                // consume all available Tokens and put the EOF Token to be read next
                self.token_buffer.clear();
                self.token_buffer.push(Token::new(
                    String::from("EOF"),
                    Some(String::from("EOF")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES1] Erro sintático na linha {}, coluna {}: nenhum código deve vir após a palavra reservada 'fim'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            // missing ';'
            2 => {
                // since a ';' is missing, put it into the input
                self.token_buffer.push(Token::new(
                    String::from("pt_v"),
                    Some(String::from(";")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES2] Erro sintático na linha {}, coluna {}: ausência de ';'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            // two or more ';' in sequence
            3 => {
                // remove one ';' at a time
                self.token_buffer.pop();

                self.syntatic_error_msgs.push(format!(
                    "[ES3] Erro sintático na linha {}, coluna {}: múltiplos ';' na sequência",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            // invalid token after a ';'
            4 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.syntatic_error_msgs.push(format!(
                    "[ES4] Erro sintático na linha {}, coluna {}: token inválido após um ';'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // '(' expected after a 'se' keyword
            5 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.token_buffer.push(Token::new(
                    String::from("ab_p"),
                    Some(String::from("(")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES5] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'se'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // '(' expected after a 'se' keyword, but and 'id' or a 'num' was found
            6 => {
                self.token_buffer.push(Token::new(
                    String::from("ab_p"),
                    Some(String::from("(")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES6] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'se'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            // opr, opm, ')' or ';' expected after a 'id'
            7 => {
                self.syntatic_error_msgs.push(format!(
                    "[ES7] Erro sintático na linha {}, coluna {}: após um identificador deve vir um operador relacional, um operador aritimético, um ')' ou um ';'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            // opr, opm, ')' or ';' expected after a 'num'
            8 => {
                self.syntatic_error_msgs.push(format!(
                    "[ES8] Erro sintático na linha {}, coluna {}: após um número deve vir um operador relacional, um operador aritimético, um ')' ou um ';'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            // 'id'/'num' not found after a 'se ('
            9 => {
                // remove the wrong token
                let token = self.token_buffer.pop().unwrap();

                if token.class.eq("fc_p") {
                    // '()' cannot be recovered
                    self.syntatic_error_msgs.push(format!(
                        "[ES9.1] Erro sintático na linha {}, coluna {}: encontrado um () após a palavra reservada 'se'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                        scanner.get_row(),
                        scanner.get_col()
                    ));
                    return false;
                } else {
                    self.syntatic_error_msgs.push(format!(
                        "[ES9.2] Erro sintático na linha {}, coluna {}: esperado um 'id' ou um 'num' após um 'se ('\n    NOTA: o token '{}' foi removido",
                        scanner.get_row(),
                        scanner.get_col(),
                        token.lexeme.unwrap()
                    ));
                    return true;
                }
            }
            // 'opr' not found after the 1st argument in a relacional expression
            10 => {
                self.syntatic_error_msgs.push(format!(
                    "[ES10] Erro sintático na linha {}, coluna {}: não encontrado '<', '>', '>=', '<=', '=' ou '<>' após o primeiro argumento de uma expressão relacional\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            // 'num' nor 'id' found after a 'opr'
            11 => {
                self.syntatic_error_msgs.push(format!(
                    "[ES11] Erro sintático na linha {}, coluna {}: esperado um 'num' ou um 'id' após um operador relacional\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            // opr, opm or ';' found after a relacional expression
            12 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.syntatic_error_msgs.push(format!(
                    "[ES12] Erro sintático na linha {}, coluna {}: após uma expressão relacional, é esperado um ')'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // some token but not 'entao' ater a 'se ( EXP_R )'
            13 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.syntatic_error_msgs.push(format!(
                    "[ES13] Erro sintático na linha {}, coluna {}: esperado a palavra reservada 'entao' após a expressão relacional de uma estrutura condicional\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // 'leia', 'escreva', 'id', 'se' or 'fimse' a 'entao'
            14 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.syntatic_error_msgs.push(format!(
                    "[ES14] Erro sintático na linha {}, coluna {}: esperado 'leia', 'escreva', 'id', 'se' ou 'fimse' após a palavra reservada 'entao'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // '(' expected after a 'repita' keyword
            15 => {
                // remove the wrong token
                let token = self.token_buffer.pop();

                self.token_buffer.push(Token::new(
                    String::from("ab_p"),
                    Some(String::from("(")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES15] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'repita'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            // '(' expected after a 'repita' keyword, but and 'id' or a 'num' was found
            16 => {
                self.token_buffer.push(Token::new(
                    String::from("ab_p"),
                    Some(String::from("(")),
                    None,
                ));

                self.syntatic_error_msgs.push(format!(
                    "[ES16] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'repita'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            // 'id'/'num' not found after a 'repita ('
            17 => {
                // remove the wrong token
                let token = self.token_buffer.pop().unwrap();

                if token.class.eq("fc_p") {
                    // '()' cannot be recovered
                    self.syntatic_error_msgs.push(format!(
                        "[ES17.1] Erro sintático na linha {}, coluna {}: encontrado um () após a palavra reservada 'repita'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                        scanner.get_row(),
                        scanner.get_col()
                    ));
                    return false;
                } else {
                    self.syntatic_error_msgs.push(format!(
                        "[ES17.2] Erro sintático na linha {}, coluna {}: esperado um 'id' ou um 'num' após um 'repita ('\n    NOTA: o token '{}' foi removido",
                        scanner.get_row(),
                        scanner.get_col(),
                        token.lexeme.unwrap()
                    ));
                    return true;
                }
            }
            _ => {
                self.syntatic_error_msgs.push(format!(
                    "[ES0] Erro sintático na linha {}, coluna {}\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
        }
    }

    fn show_syntatic_error_msgs(&self) -> u8 {
        let n = self.syntatic_error_msgs.len();
        match n {
            0 => (),
            1 => println!("Foi encontrado 1 erro sintático"),
            _ => println!("Foi encontrado {} erros sintáticos", n),
        }

        for i in 0..n {
            let msg = &self.syntatic_error_msgs[i];
            println!("# ERRO {}", i + 1);
            println!("    {}", msg);
        }

        n as u8
    }

    fn show_semantic_error_msgs(&self) -> u8 {
        let n = self.semantic_error_msgs.len();
        match n {
            0 => (),
            1 => println!("Foi encontrado 1 erro semântico"),
            _ => println!("Foi encontrado {} erros semânticos", n),
        }

        for i in 0..n {
            let msg = &self.semantic_error_msgs[i];
            println!("# ERRO {}", i + 1);
            println!("    {}", msg);
        }

        n as u8
    }
}
