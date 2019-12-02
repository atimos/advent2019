#[derive(Debug, PartialEq, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// {0}
    Parse(#[from] std::num::ParseIntError),
    /// Unknown op {op} in position {pos}
    Op { op: usize, pos: usize },
    /// Invalid argument count for op in position {0}
    ArgCount(usize),
    /// Unexpected end of file
    Eof,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn run(program: &str) -> Result<String> {
    let mut pos = 0;
    let mut program: Vec<usize> =
        program.split(',').map(str::parse).collect::<std::result::Result<_, _>>()?;

    loop {
        pos += match program.get(pos) {
            Some(1) => apply(std::ops::Add::add, pos, &mut program)?,
            Some(2) => apply(std::ops::Mul::mul, pos, &mut program)?,
            Some(99) => break,
            Some(op) => return Err(Error::Op { op: *op, pos }),
            None => return Err(Error::Eof),
        }
    }

    Ok(program.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(","))
}

fn apply(op: impl Fn(usize, usize) -> usize, pos: usize, program: &mut [usize]) -> Result<usize> {
    if let Some([first, second, output]) = &program.get(pos + 1..=pos + 3) {
        program[*output] = op(program[*first], program[*second]);
        Ok(4)
    } else {
        Err(Error::ArgCount(pos))
    }
}
