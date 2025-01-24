use std::error::Error;
use std::fmt::{self, Formatter};
use std::rc::Rc;

use crate::lexer::Token;
use crate::lsymc::Lsymc;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    NotList,
    ShortList,
}

impl fmt::Display for ParserError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        todo!()
    }
}

impl Error for ParserError {}

pub fn parse(toks: Vec<Token>) -> Result<Lsymc, ParserError> {
    let mut toks = toks.into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_exp(&mut toks)?;
    Ok(parsed_list)
}

/// It's a cons List list(items, ..., list(items, ...))
fn parse_exp(toks: &mut Vec<Token>) -> Result<Lsymc, ParserError> {
    let curr_token = toks.pop();
    if curr_token != Some(Token::LParen) {
        return Err(ParserError::NotList);
    }

    let mut list: Vec<Lsymc> = Vec::new();
    while !toks.is_empty() {
        let tok = toks.pop();
        if tok.is_none() {
            return Err(ParserError::ShortList);
        }

        let t = tok.unwrap();
        match t {
            Token::Int(i) => list.push(Lsymc::Int(i)),
            Token::Def => list.push(Lsymc::Def),
            Token::Defn => list.push(Lsymc::Defn),
            Token::Plus => list.push(Lsymc::Plus),
            Token::Minus => list.push(Lsymc::Minus),
            Token::Astrisk => list.push(Lsymc::Astrisk),
            Token::Slash => list.push(Lsymc::Slash),
            Token::Equal => list.push(Lsymc::Equal),
            Token::NotEqual => list.push(Lsymc::NotEqual),
            Token::LT => list.push(Lsymc::LT),
            Token::GT => list.push(Lsymc::GT),
            Token::Ident(i) => list.push(Lsymc::Ident(i)),
            Token::String(s) => list.push(Lsymc::String(s)),
            Token::Float(f) => list.push(Lsymc::Float(f)),
            Token::If => list.push(Lsymc::If),
            Token::Else => list.push(Lsymc::Else),
            Token::LParen => {
                toks.push(Token::LParen);
                let sub_list = parse_exp(toks)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Lsymc::List(Rc::new(list)));
            }
        }
    }
    Ok(Lsymc::List(Rc::new(list)))
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::lsymc::Lsymc;

    use super::*;

    #[test]
    fn test_parser() {
        let input = "(def x (+ 10 21))";

        let lexer = Lexer::lex(input);
        let prog = parse(lexer).unwrap();
        assert_eq!(
            prog,
            Lsymc::List(Rc::new(vec![
                Lsymc::Def,
                Lsymc::Ident("x".to_owned()),
                Lsymc::List(Rc::new(vec![Lsymc::Plus, Lsymc::Int(10), Lsymc::Int(21),])),
            ]))
        );
    }

    #[test]
    fn test_multiple_expressions() {
        let input: &str = r#"
            (defn add (x y) ( + x y ))
        "#;
        let lexer = Lexer::lex(input);
        let prog = parse(lexer).unwrap();

        assert_eq!(
            prog,
            Lsymc::List(Rc::new(vec![
                Lsymc::Defn,
                Lsymc::Ident("add".to_owned()),
                Lsymc::List(Rc::new(vec![
                    Lsymc::Ident("x".to_owned()),
                    Lsymc::Ident("y".to_owned()),
                ])),
                Lsymc::List(Rc::new(vec![
                    Lsymc::Plus,
                    Lsymc::Ident("x".to_owned()),
                    Lsymc::Ident("y".to_owned()),
                ])),
            ]))
        );
    }

    #[test]
    fn test_not_a_list() {
        let input: &str = "def x 10";
        let lexer = Lexer::lex(input);
        let prog = parse(lexer).err().unwrap();
        assert_eq!(prog, ParserError::NotList);
    }

    #[test]
    #[should_panic]
    fn not_balanced() {
        let input: &str = "(def name \"Marwan\" ";
        let lexer = Lexer::lex(input);
        let _prog = parse(lexer).unwrap();
    }
}
