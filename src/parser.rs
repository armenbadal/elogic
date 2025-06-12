use std::iter::Peekable;
use std::str::Chars;

use crate::scheme::{Design, Instruction, Schematic};

#[derive(Debug, PartialEq)]
enum Token {
    Define,
    End,
    Arrow,
    Ident(String),
    Eof,
    Unknown(char),
}

struct Scanner<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Scanner<'a> {
    fn new(input: Chars<'a>) -> Self {
        Scanner { input: input.peekable() }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        match self.input.peek() {
            Some(c) if c.is_alphabetic() => {
                Some(self.keyword_or_identifier())
            },
            Some('-') => {
                self.input.next();
                if self.input.peek().copied() == Some('>') {
                    self.input.next();
                    Some(Token::Arrow)
                } else {
                    Some(Token::Unknown('?'))
                }
            },
            Some('0') | Some('1') => {
                let v = self.input.next().unwrap();
                Some(Token::Ident(v.to_string()))
            }
            Some(c) => Some(Token::Unknown(*c)),
            _ => Some(Token::Eof)
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.input.peek() {
            if !c.is_whitespace() { break }
            self.input.next();
        }
    }

    fn keyword_or_identifier(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(c) = self.input.peek() {
            if !c.is_alphabetic() { break }
            lexeme.push(self.input.next().unwrap());
        }

        match lexeme.as_str() {
            "module" => Token::Define,
            "end" => Token::End,
            _ => Token::Ident(lexeme)
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>
}

impl<'a> Parser<'a> {
    pub fn new() {}

    pub fn parse(&self) -> Result<Design,String> {
        todo!()
    }

    fn parse_schematics(&mut self) -> Result<Schematic,String> {
        todo!()
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>,String> {
        todo!()
    }

    fn parse_identifier(&mut self) -> Result<String,String> {
        todo!()
    }

    fn parse_instruction_list(&mut self) -> Result<Vec<Instruction>,String> {
        todo!()
    }

    fn parse_instruction(&mut self) -> Result<Instruction,String> {
        todo!()
    }

    fn expect(&mut self, expected: Token) -> Result<(),String> {
        match self.scanner.peek() {
            Some(t) if *t == expected => Ok(()),
            Some(t) => Err("".to_string()),
            None => Err("".to_string()),
        }
    }
}


mod test {
    use crate::parser::Scanner;

    #[test]
    fn test_scanner() {
        let example0 = "\n\nmodule xor a b -> x\nend\n";
        let mut scanner = Scanner::new(example0.chars());
        while let Some(tok) = scanner.next_token() {
            println!("=> {:?}", tok)
        }
    }
}