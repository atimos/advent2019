pub fn step1() {
    let result = (245318..=765747)
        .map(|number| number.to_string().chars().collect::<Vec<char>>())
        .filter(|d| d.windows(2).all(is_not_decr) && d.iter().group_count().any(|c| c > 1))
        .count();
    dbg!(result);
}

pub fn step2() {
    let result = (245318..=765747)
        .map(|number| number.to_string().chars().collect::<Vec<char>>())
        .filter(|d| d.windows(2).all(is_not_decr) && d.iter().group_count().any(|c| c == 2))
        .count();
    dbg!(result);
}

fn is_not_decr(numbers: &[char]) -> bool {
    match numbers {
        &[n1, n2] => n1 <= n2,
        _ => true,
    }
}

struct GroupCount<I: Iterator>(std::iter::Peekable<I>);

impl<Iter: Iterator> Iterator for GroupCount<Iter>
where
    Iter::Item: std::cmp::PartialEq,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 1;

        while let Some(item) = self.0.next() {
            match self.0.peek() {
                Some(next) if next == &item => count += 1,
                _ => return Some(count),
            }
        }

        None
    }
}

trait GroupCountIter: Iterator {
    fn group_count(self) -> GroupCount<Self>
    where
        Self: Sized,
    {
        GroupCount(self.peekable())
    }
}

impl<T> GroupCountIter for T where T: Iterator {}
