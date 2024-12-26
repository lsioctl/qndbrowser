use std::env;
use std::fs;
use std::os::linux::raw;

#[derive(Debug)]
enum Token {
    Dot,
    LeftParen,
    RighParen,
    SemiColon,
    SingleQuote,
    Unknown,
    Let,
    Const,
    Identifier(String),
    StringLitteral(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    StartedKeywordOrIdentifier,
    StartedString,
    Stopped,
    Error,
}

fn state_transisition(state: &State, c: char) -> State {
    match state {
        State::StartedString => match c {
            '"' => State::Stopped,
            _ => state.clone(),
        },
        State::StartedKeywordOrIdentifier => match c {
            x if !x.is_alphabetic() => State::Stopped,
            _ => state.clone(),
        },
        State::Stopped => match c {
            x if x.is_alphabetic() => State::StartedKeywordOrIdentifier,
            '"' => State::StartedString,
            _ => state.clone(),
        },
        _ => State::Error,
    }
}

fn get_sep_token(ch: char) -> Token {
    match ch {
        '(' => Token::LeftParen,
        ')' => Token::RighParen,
        ';' => Token::SemiColon,
        '.' => Token::Dot,
        '\'' => Token::SingleQuote,
        _ => Token::Unknown,
    }
}

fn tokenizer(javascript: &str) -> Vec<Token> {
    let mut token_list: Vec<Token> = Vec::new();
    javascript
        // TODO: CR is not a js keyword, but I do not want to deal with ASI for now
        .split("\n")
        .enumerate()
        .for_each(|(line_nb, line)| {
            //println!("{}", line);
            let mut state = State::Stopped;
            let mut raw_token = String::from("");
            line.chars().enumerate().for_each(|(col_nb, ch)| {
                let next_state = state_transisition(&state, ch);
                // println!("{:?}, {:?}", state, next_state);
                match (&state, &next_state) {
                    (State::Stopped, State::StartedKeywordOrIdentifier) => {
                        raw_token.push(ch);
                    }
                    (State::StartedKeywordOrIdentifier, State::StartedKeywordOrIdentifier) => {
                        raw_token.push(ch);
                    }
                    (State::Stopped, State::StartedString) => {
                        // do nothing
                    }
                    (State::StartedString, State::StartedString) => {
                        raw_token.push(ch);
                    }
                    (State::StartedString, State::Stopped) => {
                        token_list.push(Token::StringLitteral(raw_token.clone()));
                        raw_token = String::from("");
                    }
                    (State::StartedKeywordOrIdentifier, State::Stopped) => {
                        // take care of the token until the stop
                        let finished_token = match &raw_token {
                            // can't compare &str to String so I use this
                            // https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match
                            s if s == "let" => Token::Let,
                            s if s == "const" => Token::Const,
                            // TODO: ugly again, strings and slice are not an easy beasts in Rust
                            // here we have a move because String has not copy trait
                            // also, raw_token is a captured outer variable in this FnMut closure
                            _ => Token::Identifier(raw_token.clone()),
                        };
                        token_list.push(finished_token);
                        raw_token = String::from("");

                        // Now treat the current token
                        token_list.push(get_sep_token(ch));
                    }
                    (State::Stopped, State::Stopped) => {
                        match ch {
                            ' ' => {
                                // do nothing
                            }
                            _ => token_list.push(get_sep_token(ch)),
                        };
                    }
                    (_, _) => {
                        // panic!("Unmatched state")
                    }
                };
                state = next_state;
            });
        });
    token_list
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let token_list = tokenizer(&content);

    println!("{:?}", token_list);
}
