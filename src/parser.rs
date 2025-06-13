use std::iter::Peekable;
use std::str::Chars;

use crate::scheme::{Design, Instruction, Schematic};

#[derive(Debug, PartialEq)]
enum Token {
    Define,
    End,
    Arrow,
    NewLine,
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
            Some('0') | Some('1') => {
                let v = self.input.next().unwrap();
                Some(Token::Ident(v.to_string()))
            }
            Some('-') => {
                self.input.next();
                if self.input.peek().copied() == Some('>') {
                    self.input.next();
                    Some(Token::Arrow)
                } else {
                    Some(Token::Unknown('?'))
                }
            },
            Some('\n') => {
                self.input.next();
                Some(Token::NewLine)
            },
            Some(c) => Some(Token::Unknown(*c)),
            None => Some(Token::Eof)
        }
    }

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.input.peek() {
            if c != &' ' && c != &'\t' { break }
            self.input.next();
        }
    }

    fn keyword_or_identifier(&mut self) -> Token {
        let mut lexeme = String::new();
        while let Some(c) = self.input.peek() {
            if !c.is_alphanumeric() { break }
            lexeme.push(self.input.next().unwrap());
        }

        match lexeme.as_str() {
            "define" => Token::Define,
            "end" => Token::End,
            _ => Token::Ident(lexeme)
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self { scanner: Scanner::new(input).peekable() }
    }

    pub fn parse(&mut self) -> Result<Design,String> {
        self.parse_newlines();

        let mut schematics = Vec::new();
        while let Some(Token::Define) = self.scanner.peek() {
            schematics.push(self.parse_schematics()?);
        }

        Ok(Design::new(schematics))
    }

    fn parse_schematics(&mut self) -> Result<Schematic,String> {
        self.expect(Token::Define)?;
        let name = self.parse_identifier()?;
        let inputs = self.parse_identifier_list()?;
        self.expect(Token::Arrow)?;
        let outputs = self.parse_identifier_list()?;
        self.parse_newlines();
        let body = self.parse_instruction_list()?;
        self.expect(Token::End)?;
        self.parse_newlines();

        Ok(Schematic::new(name, inputs, outputs, body))
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<String>,String> {
        let mut identifiers = Vec::new();
        while let Some(Token::Ident(_)) = self.scanner.peek() {
            identifiers.push(self.parse_identifier()?);
        }
        Ok(identifiers)
    }

    fn parse_identifier(&mut self) -> Result<String,String> {
        match self.scanner.next() {
            Some(Token::Ident(value)) => Ok(value),
            Some(token) => Err(format!("Expected Identifier, found {:?}", token)),
            None => Err("Expected identifier, but found end of input".into()),
        }
    }

    fn parse_instruction_list(&mut self) -> Result<Vec<Instruction>,String> {
        let mut instructions = Vec::new();
        while let Some(Token::Ident(_)) = self.scanner.peek() {
            instructions.push(self.parse_instruction()?);
        }
        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<Instruction,String> {
        let schematic_name = self.parse_identifier()?;
        let inputs = self.parse_identifier_list()?;
        self.expect(Token::Arrow)?;
        let outputs = self.parse_identifier_list()?;
        self.parse_newlines();

        Ok(Instruction::new(schematic_name, inputs, outputs))
    }

    fn parse_newlines(&mut self) {
        while let Some(Token::NewLine) =  self.scanner.peek() {
            self.scanner.next();
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(),String> {
        match self.scanner.next() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?}, got {:?}", expected, token)),
            None => Err(format!("Expected {:?}, but found end of input", expected)),
        }
    }
}


mod test {
    use crate::parser::{Parser, Scanner, Token};
    use crate::scheme::Design;

    #[test]
    fn test_scanner() {
        let example0 = "\n\ndefine xor a b -> x\nend\n";
        let mut scanner = Scanner::new(example0.chars());

        let tokens = vec![
            Token::NewLine,
            Token::NewLine,
            Token::Define,
            Token::Ident("xor".to_string()),
            Token::Ident("a".to_string()),
            Token::Ident("b".to_string()),
            Token::Arrow,
            Token::Ident("x".to_string()),
            Token::NewLine,
            Token::End,
            Token::NewLine
        ];

        for token in tokens {
            let st = scanner.next_token().unwrap();
            assert_eq!(token, st);
        }
    }

    #[test]
    fn test_parser() {
        let example1 = "\n\ndefine xor a b -> x\n\
                                nand a a -> t0\n\
                                nand b b -> t1\n\
                                nand t0 t1 -> x\n\
                               end\n";
        let mut parser = Parser::new(example1.chars());
        match parser.parse() {
            Ok(design) => { println!("=> {:#?}", design) }
            Err(e) =>  { println!("=> {:?}", e) }
        }
    }
}