use itertools::Itertools;

pub fn step1() {
    let result = (245318..=765747)
        .map(|number| {
            number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>()
        })
        .filter(|digits| digits.windows(2).all(is_not_decr) && digits.windows(2).any(is_same))
        .count();
    dbg!(result);
}

pub fn step2() {
    let result = (245318..=765747)
        .map(|number| {
            number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>()
        })
        .filter(|digits| digits.windows(2).all(is_not_decr) && has_grouped_same_two(digits))
        .count();
    dbg!(result);
}

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

fn has_grouped_same_two(numbers: &[u32]) -> bool {
    numbers.iter().peekable().batching(|iter| {
        let mut count = 1;
        match iter.next() {
            None => None,
            Some(x) => {
                while let Some(y) = iter.peek() {
                    if y != &x {
                        return Some(count);
                    }
                    iter.next();
                    count += 1;
                }
                Some(count)
            }
        }
    }).any(|count| count == 2)
}
