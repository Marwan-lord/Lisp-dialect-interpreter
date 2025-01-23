use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Def,
    Defn,
    Plus,
    Minus,
    Astrisk,
    Slash,
    LT,
    GT,
    Equal,
    NotEqual,
    String(String),
    Ident(String),
    Int(i64),
    Float(f64),
    LParen,
    RParen,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    curr: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut input = input.chars();
        let curr = input.next();

        Self { input, curr }
    }

    fn next(&mut self) -> Option<char> {
        self.curr = self.input.next();
        self.curr
    }

    fn spit_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.curr? {
            '(' => {
                self.next();
                Some(Token::LParen)
            }

            ')' => {
                self.next();
                Some(Token::RParen)
            }

            '+' => {
                self.next();
                Some(Token::Plus)
            }

            '-' => {
                self.next();
                Some(Token::Minus)
            }

            '/' => {
                self.next();
                Some(Token::Slash)
            }

            '*' => {
                self.next();
                Some(Token::Astrisk)
            }
            '<' => {
                self.next();
                Some(Token::LT)
            }

            '>' => {
                self.next();
                Some(Token::GT)
            }

            '=' => {
                self.next();
                Some(Token::Equal)
            }

            '!' => {
                self.next();
                Some(Token::NotEqual)
            }

            '"' => Some(Token::String(self.read_string())),

            c if c.is_alphabetic() => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "def" => Some(Token::Def),
                    "defn" => Some(Token::Defn),
                    _ => Some(Token::Ident(ident)),
                }
            }

            c if c.is_numeric() => {
                let result = self.read_numeric();
                if result.contains('.') {
                    Some(Token::Float(
                        result.parse::<f64>().expect("Error parsing float"),
                    ))
                } else {
                    Some(Token::Int(
                        result.parse::<i64>().expect("Error parsing int"),
                    ))
                }
            }

            _ => None,
        }
    }

    fn read_string(&mut self) -> String {
        let mut base = String::new();
        self.next();
        while let Some(c) = self.curr {
            if c == '"' {
                self.next();
                break;
            }
            base.push(c);
            self.next();
        }
        base
    }

    fn read_numeric(&mut self) -> String {
        let mut num = String::new();

        while let Some(n) = self.curr {
            if !n.is_numeric() && n != '.' {
                break;
            }
            num.push(n);
            self.next();
        }
        num
    }
    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.curr {
            if c.is_whitespace() || c == ')' || c == '(' {
                break;
            }
            ident.push(c);
            self.next();
        }
        ident
    }

    fn skip_whitespace(&mut self) {
        while let Some(w) = self.curr {
            if !w.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    pub fn lex(input: &str) -> Option<Vec<Token>> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.spit_token() {
            tokens.push(token);
        }
        Some(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let input: &str = "(def name \"Marwan\")";
        let lexer = Lexer::lex(input).unwrap();

        assert_eq!(
            lexer,
            vec![
                Token::LParen,
                Token::Def,
                Token::Ident("name".to_owned()),
                Token::String("Marwan".to_owned()),
                Token::RParen,
            ]
        )
    }
    #[test]
    fn test_int() {
        let input: &str = "(def x 10)";
        let lexer = Lexer::lex(input).unwrap();

        assert_eq!(
            lexer,
            vec![
                Token::LParen,
                Token::Def,
                Token::Ident("x".to_owned()),
                Token::Int(10),
                Token::RParen,
            ]
        )
    }

    #[test]
    fn test_float() {
        let input: &str = "(def y 4.123 + / * < >)";

        let lexer = Lexer::lex(input).unwrap();
        assert_eq!(
            lexer,
            vec![
                Token::LParen,
                Token::Def,
                Token::Ident("y".to_owned()),
                Token::Float(4.123),
                Token::Plus,
                Token::Slash,
                Token::Astrisk,
                Token::LT,
                Token::GT,
                Token::RParen,
            ]
        )
    }

    #[test]
    fn test_function() {
        let input: &str = "(defn add (x y) (x + y))";
        let lexer = Lexer::lex(input).unwrap();

        assert_eq!(
            lexer,
            vec![
                Token::LParen,
                Token::Defn,
                Token::Ident("add".to_owned()),
                Token::LParen,
                Token::Ident("x".to_owned()),
                Token::Ident("y".to_owned()),
                Token::RParen,
                Token::LParen,
                Token::Ident("x".to_owned()),
                Token::Plus,
                Token::Ident("y".to_owned()),
                Token::RParen,
                Token::RParen,
            ]
        )
    }
}
