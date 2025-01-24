use std::fmt::{Error, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Lsymc {
    Nil,
    Def,
    Defn,
    Int(i64),
    Float(f64),
    Plus,
    Minus,
    Astrisk,
    Slash,
    LT,
    GT,
    Equal,
    NotEqual,
    If,
    Else,
    // Shared Immutable refrence to the list which is
    // nice since it won't change after parsing it
    List(Rc<Vec<Lsymc>>),
    ListSyms(Vec<Lsymc>),
    Bool(bool),
    Ident(String),
    String(String),
}

impl std::fmt::Display for Lsymc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Lsymc::Nil => write!(f, "nil"),
            Lsymc::Def => write!(f, "def"),
            Lsymc::Defn => write!(f, "defn"),
            Lsymc::If => write!(f, "if"),
            Lsymc::Else => write!(f, "else"),
            Lsymc::Plus => write!(f, "+"),
            Lsymc::Minus => write!(f, "-"),
            Lsymc::Astrisk => write!(f, "*"),
            Lsymc::Slash => write!(f, "/"),
            Lsymc::LT => write!(f, "<"),
            Lsymc::GT => write!(f, ">"),
            Lsymc::Equal => write!(f, "="),
            Lsymc::NotEqual => write!(f, "!"),
            Lsymc::Bool(b) => write!(f, "{}", b),
            Lsymc::Int(i) => write!(f, "{}", i),
            Lsymc::Float(_f) => write!(f, "{}", _f),
            Lsymc::String(s) => write!(f, "{}", s),
            Lsymc::Ident(s) => write!(f, "{}", s),
            Lsymc::List(lst) => {
                write!(
                    f,
                    "[{}]",
                    lst.as_ref()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }

            Lsymc::ListSyms(lst) => {
                write!(
                    f,
                    "[{}]",
                    lst.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}
