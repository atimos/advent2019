pub fn run<R: std::io::Read>(mut input: R) -> String {
    let mut program = String::new();
    input.read_to_string(&mut program).expect("Could not load program");
    program
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .map(|mut program| {
            let mut pos = 0;
            loop {
                if let Some(99) | None = program.get(pos) {
                    break;
                }

                match program.get(pos..=pos + 3) {
                    Some(&[1, in1, in2, out]) => program[out] = program[in1] + program[in2],
                    Some(&[2, in1, in2, out]) => program[out] = program[in1] * program[in2],
                    Some(_) => panic!("Invalid operation"),
                    None => panic!("Unexpected end of file"),
                };
                pos += 4;
            }
            program
        })
        .map(|program| itertools::join(program.iter(), ","))
        .expect("Invalid program")
}
