pub fn step1<R: std::io::Read>(input: R) {
    println!("day 2 step 1 {}", run(parse(input))[0]);
}

pub fn step2<R: std::io::Read>(input: R) {
    let program = parse(input);
    let (verb, noun) = (0..100)
        .flat_map(|verb| (0..100).map(move |noun| (verb, noun)))
        .find(|(verb, noun)| {
            let mut program = program.clone();
            program[1] = *verb;
            program[2] = *noun;
            run(program)[0] == 19690720
        })
        .expect("Did not find result");

    println!("day 2 step 2 {}", 100 * verb + noun);
}

pub fn parse<R: std::io::Read>(mut input: R) -> Vec<usize> {
    let mut program = String::new();
    input.read_to_string(&mut program).expect("Could not load program");

    program.split(',').map(str::parse).collect::<Result<Vec<usize>, _>>().expect("Invalid program")
}

pub fn run(mut program: Vec<usize>) -> Vec<usize> {
    let mut pos = 0;
    loop {
        if let Some(99) | None = program.get(pos) {
            break;
        }

        match program.get(pos..=pos + 3) {
            Some(&[1, in1, in2, out]) => program[out] = program[in1] + program[in2],
            Some(&[2, in1, in2, out]) => program[out] = program[in1] * program[in2],
            Some(&[op, _, _, _]) => panic!("Invalid operation {}", op),
            _ => panic!("Unexpected end of file"),
        };
        pos += 4;
    }
    program
}
