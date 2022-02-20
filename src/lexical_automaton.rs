// All allowed automaton states
pub enum AutomatonState {
    Initial,         // 0
    Accept(u8),      // 1, 2, ..., 18
    NonAccept(char), // a, b, c, d, e
    Error(u8),       // lexical error - 0: character isn't in the alphabet
                     //                 1: character doesn't start a token
                     //                 2: no digit after a '.' in a num token
                     //                 3: no digit, '+' or '-' after a 'e'/'E' in a num token
                     //                 4: no digit after a ('e'/'E')('+''-')  in a num token
                     //                 5: unfinished comment or literal
}

// Actions to be performed by the Scanner using the the automaton
pub enum Action {
    None,        // do nothing
    GoBack,      // make the scanner cursor to go one step back
    Standard,    // append the last character read into the lexeme
    ClearLexeme, // clear the scanner lexeme being written
    ShowError,   // show error message
}

// A struct to represent an automaton used in the lexical
// analysis. Internally, it keeps an AutomatonState (current
// automaton state), a bool variable done (when done == true
// the automaton reached an accept state) and an Action (to be
// performed by the Scanner).
pub struct Automaton {
    pub state: AutomatonState,
    pub done: bool,
    pub action: Action,
}

impl Automaton {
    // create a new Automaton
    pub fn new() -> Automaton {
        Automaton {
            state: AutomatonState::Initial,
            done: false,
            action: Action::None,
        }
    }

    // advance the automaton by the reading of a character
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
                c if is_in_alphabet(c) => self.error(1),
                _ => self.error(0),
            },
            AutomatonState::Accept(1) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(1),
                '.' => self.state = AutomatonState::NonAccept('a'),
                'e' | 'E' => self.state = AutomatonState::NonAccept('b'),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(0),
            },
            AutomatonState::Accept(2) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(2),
                'e' | 'E' => self.state = AutomatonState::NonAccept('b'),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(0),
            },
            AutomatonState::Accept(3) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(0),
            },
            AutomatonState::Accept(5) => match c {
                '0'..='9' => self.state = AutomatonState::Accept(5),
                'a'..='z' | 'A'..='Z' => self.state = AutomatonState::Accept(5),
                '_' => self.state = AutomatonState::Accept(5),
                c if is_in_alphabet(c) => {
                    self.done = true;
                    self.action = Action::GoBack;
                }
                _ => self.error(0),
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
                _ => self.error(0),
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
                _ => self.error(0),
            },
            AutomatonState::NonAccept('a') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(2),
                _ => self.error(2),
            },
            AutomatonState::NonAccept('b') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                '+' | '-' => self.state = AutomatonState::NonAccept('c'),
                _ => self.error(3),
            },
            AutomatonState::NonAccept('c') => match c {
                '0'..='9' => self.state = AutomatonState::Accept(3),
                _ => self.error(4),
            },
            AutomatonState::NonAccept('d') => match c {
                '"' => {
                    self.done = true;
                    self.state = AutomatonState::Accept(4);
                }
                c if is_in_alphabet(c) => {
                    self.state = AutomatonState::NonAccept('d');
                }
                _ => self.error(0),
            },
            AutomatonState::NonAccept('e') => match c {
                '}' => {
                    self.state = AutomatonState::Initial;
                    self.action = Action::ClearLexeme;
                }
                c if is_in_alphabet(c) => self.state = AutomatonState::NonAccept('e'),
                _ => self.error(0),
            },
            _ => (),
        }
    }

    // Put the automaton in the AutomatonState::Error state and
    // end its execution by setting self.done to true
    fn error(&mut self, kind: u8) {
        self.state = AutomatonState::Error(kind);
        self.done = true;
        self.action = Action::ShowError;
    }
}

// check if a characters is in the MGol alphabet
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
