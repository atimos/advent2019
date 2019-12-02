pub fn run(program: &str) -> String {
    let mut pos = 0;
    let mut program: Vec<usize> =
        program.split(',').map(str::parse).collect::<Result<_, _>>().expect("Invalid program");

    loop {
        pos += match program.get(pos) {
            Some(1) => apply(std::ops::Add::add, pos, &mut program),
            Some(2) => apply(std::ops::Mul::mul, pos, &mut program),
            Some(99) => break,
            Some(_) => panic!("unknown op"),
            None => panic!("eof"),
        };
    }

    program.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(",")
}

fn apply(op: impl Fn(usize, usize) -> usize, pos: usize, program: &mut [usize]) -> usize {
    if let Some([first, second, output]) = &program.get(pos + 1..=pos + 3) {
        program[*output] = op(program[*first], program[*second]);
        4
    } else {
        panic!("invalid arg count");
    }
}
