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
    UpdateLexeme,
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
        match self.state {
            AutomatonState::Initial => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(1), Action::UpdateLexeme),
                    '"' => self.go_to_state(AutomatonState::NonAccept('d'), Action::UpdateLexeme),
                    'a'..='z' | 'A'..='Z' => {
                        self.go_to_state(AutomatonState::Accept(5), Action::UpdateLexeme);
                    }
                    '{' => self.go_to_state(AutomatonState::NonAccept('e'), Action::None),
                    '<' => self.go_to_state(AutomatonState::Accept(8), Action::UpdateLexeme),
                    '>' => self.go_to_state(AutomatonState::Accept(12), Action::UpdateLexeme),
                    '=' => self.go_to_state(AutomatonState::Accept(14), Action::UpdateLexeme),
                    '+' | '-' | '*' | '/' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(15), Action::UpdateLexeme)
                    }
                    '(' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(16), Action::UpdateLexeme);
                    }
                    ')' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(17), Action::UpdateLexeme);
                    }
                    ';' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(18), Action::UpdateLexeme);
                    }
                    '\n' | '\r' | ' ' => self.go_to_state(AutomatonState::Initial, Action::None),
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(1) => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(1), Action::UpdateLexeme),
                    '.' => self.go_to_state(AutomatonState::NonAccept('a'), Action::UpdateLexeme),
                    'e' | 'E' => {
                        self.go_to_state(AutomatonState::NonAccept('b'), Action::UpdateLexeme);
                    }
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(2) => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(2), Action::UpdateLexeme),
                    'e' | 'E' => {
                        self.go_to_state(AutomatonState::NonAccept('b'), Action::UpdateLexeme);
                    }
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(3) => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(3), Action::UpdateLexeme),
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(5) => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(5), Action::UpdateLexeme),
                    'a'..='z' | 'A'..='Z' => {
                        self.go_to_state(AutomatonState::Accept(5), Action::UpdateLexeme);
                    }
                    '_' => self.go_to_state(AutomatonState::Accept(5), Action::UpdateLexeme),
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(8) => {
                match c {
                    '=' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(9), Action::UpdateLexeme);
                    }
                    '>' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(10), Action::UpdateLexeme);
                    }
                    '-' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(11), Action::UpdateLexeme);
                    }
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::Accept(12) => {
                match c {
                    '=' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(13), Action::UpdateLexeme);
                    }
                    c if is_in_alphabet(c) => {
                        self.done = true;
                        self.action = Action::GoBack;
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::NonAccept('a') => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(2), Action::UpdateLexeme),
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::NonAccept('b') => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(3), Action::UpdateLexeme),
                    '+' | '-' => {
                        self.go_to_state(AutomatonState::NonAccept('c'), Action::UpdateLexeme);
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::NonAccept('c') => {
                match c {
                    '0'..='9' => self.go_to_state(AutomatonState::Accept(3), Action::UpdateLexeme),
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::NonAccept('d') => {
                match c {
                    '"' => {
                        self.done = true;
                        self.go_to_state(AutomatonState::Accept(4), Action::UpdateLexeme);
                    }
                    c if is_in_alphabet(c) => {
                        self.go_to_state(AutomatonState::NonAccept('d'), Action::UpdateLexeme);
                    }
                    _ => self.error(),
                }
                return;
            }
            AutomatonState::NonAccept('e') => {
                match c {
                    '}' => self.go_to_state(AutomatonState::Initial, Action::None),
                    c if is_in_alphabet(c) => {
                        self.go_to_state(AutomatonState::NonAccept('e'), Action::None);
                    }
                    _ => self.error(),
                }
                return;
            }
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

    // Puts the automaton in a new state and set a new action to be performed
    fn go_to_state(&mut self, state: AutomatonState, action: Action) {
        self.state = state;
        self.action = action;
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
        '!' | '?' | '\\'  => true,
        '"' | '\'' => true,
        '\n' | '\r' | ' ' => true,
        _ => false,
    }
}
