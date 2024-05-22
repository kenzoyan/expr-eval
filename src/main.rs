use std::{fmt::Display, iter::Peekable, str::Chars};

// Token Definition
#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Mutiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
}

//Associate
const ASSOC_LEFT: i32 = 0;
const ASSOC_RIGHT: i32 = 1;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Plus => "+".to_string(),
                Token::Minus => "-".to_string(),
                Token::Mutiply => "*".to_string(),
                Token::Divide => "/".to_string(),
                Token::Power => "^".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
            }
        )
    }
}

impl Token {
    
    fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Mutiply | Token::Divide | Token::Power | Token::LeftParen | Token::RightParen => true,
            _ => false, 
        }
    }

    // get the precedence level of token
    fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Divide | Token::Mutiply => 2,
            Token::Power => 3,
            _ => 0,
        }
    }
    
    //get associative of operator
    fn assoc(&self) -> i32 {
        match self {
            Token::Power => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
        }
    }

    fn compute(&self, l: i32 , r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Mutiply => Some(l * r),
            Token::Divide => Some(l / r),
            Token::Power => Some(l.pow(r as u32)),
            _ => None
        }
    }
}

struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(expr: &'a str) -> Self {
        Self { tokens: expr.chars().peekable(), }
    }

    // Clean white spaces in expression
    fn consume_whitespaces(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    // Scan number to Token
    fn scan_numbers(&mut self ) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric(){
                num.push(c);
                self.tokens.next();
            } else {
                break;
            }
        }

        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None
        }
    }

    // Scan operators to Token
    fn scan_operator(&mut self ) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Mutiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespaces();

        // parse Token to correct type
        match self.tokens.peek() {
            Some(c) if c.is_numeric() => self.scan_numbers(),
            Some(_) => self.scan_operator(),
            None => return None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
