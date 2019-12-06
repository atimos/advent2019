pub fn step1() {
    let result = (245318..=765747)
        .map(|number| number.to_string().chars().collect::<Vec<char>>())
        .filter(|digits| digits.windows(2).all(is_not_decr) && digits.windows(2).any(is_same))
        .count();
    dbg!(result);
}

pub fn step2() {
    let result = (245318..=765747)
        .map(|number| number.to_string().chars().collect::<Vec<char>>())
        .filter(|digits| {
            digits.windows(2).all(is_not_decr)
                && digits.iter().group().any(|group| group.len() == 2)
        })
        .count();
    dbg!(result);
}

fn is_not_decr(numbers: &[char]) -> bool {
    match numbers {
        &[n1, n2] => n1 <= n2,
        _ => true,
    }
}

fn is_same(numbers: &[char]) -> bool {
    match numbers {
        &[n1, n2] => n1 == n2,
        _ => false,
    }
}

impl<T> GroupIter for T where T: Iterator {}

struct Group<Iter: Iterator>(std::iter::Peekable<Iter>);

impl<Iter: Iterator> Iterator for Group<Iter>
where
    Iter::Item: std::cmp::PartialEq + std::marker::Copy,
{
    type Item = Vec<Iter::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();
        let mut last: Option<Iter::Item> = None;

        while let Some(item) = self.0.peek() {
            if last == None || last == Some(*item) {
                group.push(*item);
                last = self.0.next();
            } else {
                break;
            }
        }

        last.map(|_| group)
    }
}

trait GroupIter: Iterator {
    fn group(self) -> Group<Self>
    where
        Self: Sized,
    {
        Group(self.peekable())
    }
}
