pub enum AutomatonState {
    Initial,         // 0
    Accept(u8),      // 1, 2, ..., 18
    NonAccept(char), // a, b, c, d, e
    Error,           // lexical error
}

pub struct Automaton {
    pub state: AutomatonState,
    pub done: bool,
    pub go_back: bool,
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            state: AutomatonState::Initial,
            done: false,
            go_back: false,
        }
    }

    pub fn advance(&mut self, c: char) {
        match self.state {
            AutomatonState::Initial => match c {
                '0'..='9' => {
                    self.state = AutomatonState::Accept(1);
                    return;
                }
                '"' => {
                    self.state = AutomatonState::NonAccept('d');
                    return;
                }
                'a'..='z' | 'A'..='Z' => {
                    self.state = AutomatonState::Accept(5);
                    return;
                }
                '{' => {
                    self.state = AutomatonState::NonAccept('e');
                    return;
                }
                '<' => {
                    self.state = AutomatonState::Accept(8);
                    return;
                }
                '>' => {
                    self.state = AutomatonState::Accept(12);
                    return;
                }
                '=' => {
                    self.state = AutomatonState::Accept(14);
                    return;
                }
                '+' | '-' | '*' | '/' => {
                    self.state = AutomatonState::Accept(15);
                    return;
                }
                '(' => {
                    self.state = AutomatonState::Accept(16);
                    return;
                }
                ')' => {
                    self.state = AutomatonState::Accept(16);
                    return;
                }
                ';' => {
                    self.state = AutomatonState::Accept(16);
                    return;
                }
                '\n' | '\r' | ' ' => return,
                _ => {
                    self.state = AutomatonState::Error;
                    return;
                }
            },
            AutomatonState::Accept(1) => {}
            AutomatonState::Accept(2) => {}
            AutomatonState::Accept(3) => {}
            AutomatonState::Accept(4) => {}
            AutomatonState::Accept(5) => {}
            AutomatonState::Accept(6) => {}
            AutomatonState::Accept(7) => {}
            AutomatonState::Accept(8) => {}
            AutomatonState::Accept(9) => {}
            AutomatonState::Accept(10) => {}
            AutomatonState::Accept(11) => {}
            AutomatonState::Accept(12) => {}
            AutomatonState::Accept(13) => {}
            AutomatonState::Accept(14) => {}
            AutomatonState::Accept(15) => {}
            AutomatonState::Accept(16) => {}
            AutomatonState::Accept(17) => {}
            AutomatonState::Accept(18) => {}
            AutomatonState::NonAccept('a') => {}
            AutomatonState::NonAccept('b') => {}
            AutomatonState::NonAccept('c') => {}
            AutomatonState::NonAccept('d') => {}
            AutomatonState::NonAccept('e') => {}
            _ => (),
        }
    }
}
