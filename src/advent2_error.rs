use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::{Add, Mul},
    result::Result as StdResult,
    string::ToString,
};

#[derive(Debug, PartialEq)]
pub enum Error {
    Parse(ParseIntError),
    Op { op: usize, pos: usize },
    ArgCount(usize),
    Eof,
}

pub type Result<T> = StdResult<T, Error>;

impl StdError for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Parse(err) => err.fmt(f),
            Self::Op { op, pos } => write!(f, "Unknown op {}, in position {}", op, pos),
            Self::ArgCount(pos) => write!(f, "Invalid argument count for op in position {}", pos),
            Self::Eof => write!(f, "Unexpected end of file"),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::Parse(err)
    }
}

pub fn run(program: &str) -> Result<String> {
    let mut pos = 0;
    let mut program = program.split(',').map(str::parse).collect::<StdResult<Vec<usize>, _>>()?;

    loop {
        pos += match program.get(pos) {
            Some(1) => apply(Add::add, pos, &mut program)?,
            Some(2) => apply(Mul::mul, pos, &mut program)?,
            Some(99) => break,
            Some(op) => return Err(Error::Op { op: *op, pos }),
            None => return Err(Error::Eof),
        }
    }

    Ok(program.iter().map(ToString::to_string).collect::<Vec<String>>().join(","))
}

fn apply(op: impl Fn(usize, usize) -> usize, pos: usize, program: &mut [usize]) -> Result<usize> {
    if let Some([first, second, output]) = &program.get(pos + 1..=pos + 3) {
        program[*output] = op(program[*first], program[*second]);
        Ok(4)
    } else {
        Err(Error::ArgCount(pos))
    }
}
