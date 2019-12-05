use itertools::Itertools;

pub fn step1() {
    dbg!((111110..111112)
        .map(|number| {
            number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>()
        })
        .find(|digits| digits.windows(2).all(is_not_decr) && digits.windows(2).any(is_same))
        .map(|digits| digits.iter().join("")));
}

pub fn step2() {}

fn is_not_decr(numbers: &[u32]) -> bool {
    match numbers {
        &[n1, n2] => n1 <= n2,
        _ => true,
    }
}

fn is_same(numbers: &[u32]) -> bool {
    match numbers {
        &[n1, n2] => n1 == n2,
        _ => false,
    }
}
