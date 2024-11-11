use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Char(char),
    Dot,
    Star,
    Plus,
    Question,
    LParen,
    RParen,
}

/// Converte il pattern in una lista di token.
fn tokenize(pattern: &str) -> Vec<Token> {
    pattern.chars().map(|c| match c {
        '.' => Token::Dot,
        '*' => Token::Star,
        '+' => Token::Plus,
        '?' => Token::Question,
        '(' => Token::LParen,
        ')' => Token::RParen,
        _ => Token::Char(c),
    }).collect()
}

#[derive(Debug)]
enum State {
    Match,
    Split(usize, usize),
    Char(char, usize),
    Dot(usize),
}

struct NFA {
    states: Vec<State>,
}

impl NFA {
    /// Costruisce l'NFA dai token, gestendo operatori regex basilari.
    fn from_tokens(tokens: &[Token]) -> Self {
        let mut states = Vec::new();
        let mut stack: Vec<usize> = Vec::new();
        let mut prev = 0;

        for token in tokens {
            match token {
                Token::Char(c) => {
                    states.push(State::Char(*c, prev + 1));
                    prev += 1;
                }
                Token::Dot => {
                    states.push(State::Dot(prev + 1));
                    prev += 1;
                }
                Token::Star => {
                    states.push(State::Split(prev + 1, prev));
                }
                Token::Plus => {
                    let last_state = prev;
                    stack.push(last_state);
                }
                Token::Question => {
                    states.push(State::Split(prev + 1, prev + 2));
                    prev += 1;
                }
                Token::LParen => stack.push(prev),
                Token::RParen => {
                    if let Some(start) = stack.pop() {
                        let end = prev;
                        states.push(State::Split(start, end));
                        prev = end;
                    }
                }
            }
        }
        states.push(State::Match);
        NFA { states }
    }

    /// Controlla se il testo corrisponde all'NFA.
    fn is_match(&self, text: &str) -> bool {
        self.search(text).is_some()
    }

    /// Cerca nel testo una corrispondenza.
    fn search(&self, text: &str) -> Option<usize> {
        let mut active_states = vec![(0, 0)];
        let text_chars: Vec<char> = text.chars().collect();

        while let Some((state_idx, text_idx)) = active_states.pop() {
            if state_idx >= self.states.len() {
                continue;
            }
            match &self.states[state_idx] {
                State::Match => return Some(text_idx),
                State::Char(c, next) => {
                    if text_idx < text_chars.len() && text_chars[text_idx] == *c {
                        active_states.push((*next, text_idx + 1));
                    }
                }
                State::Dot(next) => {
                    if text_idx < text_chars.len() {
                        active_states.push((*next, text_idx + 1));
                    }
                }
                State::Split(x, y) => {
                    active_states.push((*x, text_idx));
                    active_states.push((*y, text_idx));
                }
            }
        }
        None
    }
}

pub struct Regex {
    nfa: NFA,
}

impl Regex {
    /// Compila il pattern in un nuovo oggetto Regex.
    pub fn new(pattern: &str) -> Self {
        let tokens = tokenize(pattern);
        let nfa = NFA::from_tokens(&tokens);
        Regex { nfa }
    }

    /// Verifica se l'intero testo corrisponde al pattern.
    pub fn is_match(&self, text: &str) -> bool {
        self.nfa.is_match(text)
    }

    /// Trova la prima occorrenza del pattern nel testo.
    pub fn find(&self, text: &str) -> Option<usize> {
        self.nfa.search(text)
    }
}

fn main() {
    let patterns = vec![
        ("a.b", "acb"),
        ("a.*b", "axxxxxb"),
        ("a?b", "b"),
        ("a+b", "aaab"),
        ("(a|b)c", "bc"),
        ("a(bc)*d", "abcbcd"),
    ];

    for (pattern, text) in patterns {
        let regex = Regex::new(pattern);
        println!("Pattern: '{}', Text: '{}', Match: {}", pattern, text, regex.is_match(text));
    }

    let complex_patterns = vec![
        ("a(b|c)*d", "abcd"),
        ("a.c", "abc"),
        ("ab+c", "abbc"),
        ("(ab)?c", "c"),
        ("(a|b)*c", "ac"),
        ("a(b|c)+d", "abccd"),
        ("(a|b)+c", "aac"),
        ("a(b|c)?d", "acd"),
    ];

    for (pattern, text) in complex_patterns {
        let regex = Regex::new(pattern);
        println!("Pattern: '{}', Text: '{}', Match: {}", pattern, text, regex.is_match(text));
    }

    let additional_patterns = vec![
        ("ab*c", "ac"),
        ("a+b+c+", "aabbc"),
        ("a*b*c*", "abc"),
        ("a.b.c", "aXbYc"),
        ("a(b(c)d)e", "abcdbe"),
        ("(a|b)(c|d)", "ad"),
        ("(a|b)*", "aaabbb"),
        ("a(b|c)(d|e)", "abd"),
    ];

    for (pattern, text) in additional_patterns {
        let regex = Regex::new(pattern);
        println!("Pattern: '{}', Text: '{}', Match: {}", pattern, text, regex.is_match(text));
    }

    let special_cases = vec![
        ("", ""), 
        ("a", ""),
        ("a?", "a"),
        ("a?", ""),
        ("a*", "aaa"),
        (".*", "any text"),
    ];

    for (pattern, text) in special_cases {
        let regex = Regex::new(pattern);
        println!("Pattern: '{}', Text: '{}', Match: {}", pattern, text, regex.is_match(text));
    }
}
