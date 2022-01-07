#[derive(Debug)]
pub enum AutomatonState {
    Initial,         // 0
    Accept(u8),      // 1, 2, ..., 18
    NonAccept(char), // a, b, c, d, e
    Error,           // lexical error
}

pub enum Action {
    None,
    GoBack,
    Standard,
    ClearLexeme,
    ShowError,
}

pub struct Automaton {
    pub state: AutomatonState,
    pub done: bool,
    pub action: Action,
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            state: AutomatonState::Initial,
            done: false,
            action: Action::None,
        }
    }

    pub fn advance(&mut self, c: char) {
        self.action = Action::Standard;

        match self.state {
            AutomatonState::Initial => match c {
                '0'..='9' => self.state = AutomatonState::Accept(1),
                '"' => self.state = AutomatonState::NonAccept('d'),
                'a'..='z' | 'A'..='Z' => self.state = AutomatonState::Accept(5),
                '{' => self.state = AutomatonState::NonAccept('e'),
                '<' => self.state = AutomatonState::Accept(8),
                '>' => self.state = AutomatonState::Accept(12),
                '=' => self.state = AutomatonState::Accept(14),
                '+' | '-' | '*' | '/' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(15);
                }
                '(' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(16);
                }
                ')' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(17);
                }
                ';' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(18);
                }
                '\n' | '\r' | ' ' => {
                    self.state = AutomatonState::Initial;
                    self.action = Action::None;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(1) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(1),
                '.' => self.state = AutomatonState::NonAccept('a'),
                'e' | 'E' => self.state = AutomatonState::NonAccept('b'),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(2) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(2),
                'e' | 'E' => self.state = AutomatonState::NonAccept('b'),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(3) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(5) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(5),
                'a'..='z' | 'A'..='Z' => self.state = AutomatonState::Accept(5),
                '_' => self.state = AutomatonState::Accept(5),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(8) => match c {
                '=' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(9);
                }
                '>' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(10);
                }
                '-' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(11);
                }
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::Accept(12) => match c {
                '=' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(13);
                }
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(),
            },
            AutomatonState::NonAccept('a') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(2),
                _ => self.error(),
            },
            AutomatonState::NonAccept('b') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                '+' | '-' => self.state = AutomatonState::NonAccept('c'),
                _ => self.error(),
            },
            AutomatonState::NonAccept('c') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                _ => self.error(),
            },
            AutomatonState::NonAccept('d') => match c {
                '"' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(4);
                }
                c if is_in_alphabet(c) => {
                    self.state = AutomatonState::NonAccept('d');
                }
                _ => self.error(),
            },
            AutomatonState::NonAccept('e') => match c {
                '}' => {
                    self.state = AutomatonState::Initial;
                    self.action = Action::ClearLexeme;
                }
                c if is_in_alphabet(c) => self.state = AutomatonState::NonAccept('e'),
                _ => self.error(),
            },
            _ => (),
        }
    }

    // Puts the automaton in the AutomatonState::Error state and
    // ends its execution by setting self.done to true
    fn error(&mut self) {
        self.state = AutomatonState::Error;
        self.done = true;
        self.action = Action::ShowError;
    }
}

fn is_in_alphabet(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'a'..='z' => true,
        'A'..='Z' => true,
        ',' | '.' | ';' | ':' => true,
        '<' | '>' | '=' => true,
        '(' | ')' => true,
        '[' | ']' => true,
        '{' | '}' => true,
        '+' | '-' | '*' | '/' => true,
        '!' | '?' | '\\' => true,
        '"' | '\'' => true,
        '\n' | '\r' | ' ' => true,
        _ => false,
    }
}
