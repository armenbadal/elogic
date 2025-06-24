use std::collections::{VecDeque};
use std::fs::File;
use std::path::Path;
use std::{io::BufRead, io::BufReader};
use std::iter::Peekable;

use crate::schematic::{Design, Instruction, Schematic};

#[derive(Debug, PartialEq)]
enum Token {
    Define,
    End,
    True,
    False,
    Zero,
    One,
    Arrow,
    Ident(String),
    NewLine,
}

impl Token {
    fn from(lexeme: &str) -> Self {
        use Token::*;

        match lexeme {
            "define" => Define,
            "end" => End,
            "true" => True,
            "false" => False,
            "0" => Zero,
            "1" => One,
            "->" => Arrow,
            _ => Ident(lexeme.into())
        }
    }
}

struct Scanner {
    reader: BufReader<File>,
    tokens: VecDeque<Token>,
}

impl Scanner {
    fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        Self {
            reader: BufReader::new(file),
            tokens: VecDeque::new()
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.tokens.is_empty() {
            self.scan_next_line();
        }

        self.tokens.pop_front()
    }

    fn scan_next_line(&mut self) {
        let mut line = String::new();
        if 0 != self.reader.read_line(&mut line).unwrap() {
            let cleaned = match line.trim().find("--") {
                Some(place) => line[..place].trim_end().to_string(),
                None => line
            };
            self.tokens.extend(cleaned.split_whitespace().map(|e| Token::from(e)));
            self.tokens.push_back(Token::NewLine);
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


pub struct Parser {
    scanner: Peekable<Scanner>
}

impl Parser {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self { scanner: Scanner::new(path).peekable() }
    }

    pub fn parse(&mut self) -> Result<Design,String> {
        self.parse_design()
    }

    fn parse_design(&mut self) -> Result<Design,String> {
        self.parse_newlines();

        let mut schematics = Vec::new();
        while let Some(Token::Define) = self.scanner.peek() {
            schematics.push(self.parse_schematic()?);
        }

        Ok(Design::new(schematics))
    }

    fn parse_schematic(&mut self) -> Result<Schematic,String> {
        self.expect(Token::Define)?;
        let name = self.parse_identifier()?;
        let inputs = self.parse_identifier_list()?;
        self.expect(Token::Arrow)?;
        let outputs = self.parse_identifier_list()?;
        self.parse_newlines();
        let body = self.parse_instruction_list()?;
        self.expect(Token::End)?;
        self.parse_newlines();

        Ok(Schematic::new(name, (inputs, outputs), body))
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


#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_parse_design() {
        let src = "schematics/example01.elogic";
        let mut parser = Parser::new(src);
        match parser.parse() {
            Ok(design) => println!("=> {:#?}", design),
            Err(message) => eprintln!("ERROR: {}", message),
        }
    }
}
