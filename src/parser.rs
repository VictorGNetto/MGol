use super::scanner::Scanner;
use super::grammar::Grammar;
use super::slr_table::{ActionTable, GotoTable, SlrAction};

struct Stack {
    stack: Vec<u8>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: vec![0] }
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

pub struct Parser {
    stack: Stack,
    grammar: Grammar,
    action_table: ActionTable,
    goto_table: GotoTable,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            stack: Stack::new(),
            grammar: Grammar::new(),
            action_table: ActionTable::new(),
            goto_table: GotoTable::new(),
        }
    }

    pub fn parse(&mut self, scanner: &mut Scanner) {
        let mut a = scanner.scan().class;
        loop {
            let s = self.stack.top();
            let action = self.action_table.get(&(s, a.clone()));
            match action {
                SlrAction::S(t) => {
                    self.stack.push(t);
                    a = scanner.scan().class;
                }
                SlrAction::R(r) => {
                    let rule = self.grammar.get_rule(r as usize);
                    rule.show();
                    #[allow(non_snake_case)]
                    let A = rule.left;
                    let beta = rule.right;
                    self.stack.pop(beta.len() as u8);
                    let t = self.stack.top();
                    self.stack.push(self.goto_table.get(&(t, A.text.clone())));
                }
                SlrAction::Acc => break,
                SlrAction::E(_e) => break,
            }
        }
    }
}
