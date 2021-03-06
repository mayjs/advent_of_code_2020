use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum TokenType {
    Literal,
    Plus,
    Times,
    OpeningBracket,
    ClosingBracket,
}

struct Token<'a>(TokenType, &'a str);

struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn take_one(&mut self) -> Option<char> {
        let res = self.peek();
        if res.is_some() {
            self.pos += 1;
        }
        res
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input.as_bytes()[self.pos] as char)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.peek().map(char::is_whitespace) == Some(true) {
            self.take_one();
        }
        let start = self.pos;
        self.peek().map(|c| {
            let typ = if c.is_numeric() {
                while self.peek().map(char::is_numeric) == Some(true) {
                    self.take_one();
                }
                TokenType::Literal
            } else {
                self.take_one();
                match c {
                    '+' => TokenType::Plus,
                    '*' => TokenType::Times,
                    '(' => TokenType::OpeningBracket,
                    ')' => TokenType::ClosingBracket, 
                    _   => panic!("Unexpected input"),
                }
            };
            Token(typ, &self.input[start..self.pos])
        })
    }
}

fn tokenize(input: &str) -> impl Iterator<Item=Token> {
    Tokenizer { input, pos: 0 }
}

fn parse_lit(lit: &str) -> u64 {
    lit.parse::<u64>().expect(&format!("Could not convert {}", lit))
}

fn evaluate<'a>(input: &mut impl Iterator<Item=Token<'a>>) -> u64 {
    let mut state: u64 = 0;
    let mut last_tok: Option<Token<'a>> = None;
    while let Some(tok) = input.next() {
        let maybe_val = match tok.0 {
            TokenType::ClosingBracket => return state,
            TokenType::OpeningBracket => Some(evaluate(input)),
            TokenType::Literal => Some(parse_lit(tok.1)),
            TokenType::Plus | TokenType::Times => None,
        };
        if let Some(val) = maybe_val {
            if last_tok.as_ref().map(|t| t.0 == TokenType::Plus).unwrap_or(true) {
                state += val;
            } else if last_tok.as_ref().map(|t| t.0 == TokenType::Times).unwrap_or(false) {
                state *= val;
            } else {
                panic!("Unexpected last tok");
            }
        }
        last_tok = Some(tok);
    }
    
    state
}

fn main() {
    let stdin = io::stdin();
    let sum = stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|l| evaluate(&mut tokenize(&l)))
        .sum::<u64>();
    println!("Total sum is: {}", sum);
}