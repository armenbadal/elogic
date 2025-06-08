use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    Module,
    End,
    True,
    False,
    Zero,
    One,
    Arrow,
    Ident(String),
    Eof
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
            Some(c) if c.is_alphabetic() => Some(self.keyword_or_identifier()),
            Some(c) if c == &'-' => None,
            _ => None,
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
            "module" => Token::Module,
            "end" => Token::End,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(lexeme)
        }
    }
}

mod test {
    use crate::parser::Scanner;

    #[test]
    fn test_scanner() {
        let example0 = "\n\nmodule xor a b -> x\nend\n";
        let mut scanner = Scanner::new(example0.chars());
        let tok = scanner.next_token();
        println!("=> {:?}", tok)
    }
}