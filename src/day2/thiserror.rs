#[derive(Debug, PartialEq, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// {0}
    Parse(#[source] std::num::ParseIntError),
    /// Unknown op {op} in position {pos}
    Op { op: usize, pos: usize },
    /// Unexpected end of file
    Eof,
    /// Address {0} out of bounds
    OutOfBounds(usize),
    /// Could not finish program, max operations {0}
    ToManyOperations(usize),
}

pub type Result<T> = std::result::Result<T, Error>;

const MAX_OPERATIONS: usize = 1000;

pub fn run(program: &str) -> Result<String> {

    program
        .split(',')
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()
        .map_err(Error::Parse)
        .and_then(|mut program: Vec<usize>| {
            let mut pos = 0;
            for _ in 0..=MAX_OPERATIONS {
                match program.get(pos) {
                    Some(99) | None => return Ok(program),
                    Some(op) if *op != 1 && *op != 2 => return Err(Error::Op { op: *op, pos }),
                    _ => {}
                }

                match program.get(pos..=pos + 3) {
                    Some(&[1, in1, in2, out]) => {
                        apply(std::ops::Add::add, in1, in2, out, &mut program)?
                    }
                    Some(&[2, in1, in2, out]) => {
                        apply(std::ops::Mul::mul, in1, in2, out, &mut program)?
                    }
                    None => return Err(Error::Eof),
                    Some(_) => unreachable!(),
                };
            dbg!(pos);
                pos += 4;
            }
            Err(Error::ToManyOperations(MAX_OPERATIONS))
        })
        .map(|program| itertools::join(program.iter(), ","))
}

fn apply(
    op: impl Fn(usize, usize) -> usize,
    in1: usize,
    in2: usize,
    out: usize,
    program: &mut [usize],
) -> Result<()> {
    let in1 = *program.get(in1).ok_or(Error::OutOfBounds(in1))?;
    let in2 = *program.get(in2).ok_or(Error::OutOfBounds(in2))?;
    let out = program.get_mut(out).ok_or(Error::OutOfBounds(out))?;
    *out = op(in1, in2);
    Ok(())
}
