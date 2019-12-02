#[allow(clippy::all)]
mod advent2_enum;
mod advent2_error;
mod advent2_panic;
mod advent2_thiserror;

fn main() {
    advent2_panic::run("99");
    advent2_enum::run("99").unwrap();
    advent2_error::run("99").unwrap();
    advent2_thiserror::run("99").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advent1() {
        for (input, output) in advent1_input() {
            assert_eq!((*input as f32 / 3.).floor() - 2., *output);
        }
    }

    #[test]
    fn advent2_panic() {
        for (input, output) in advent2_input() {
            assert_eq!(output.to_string(), advent2_panic::run(input))
        }
    }

    #[test]
    fn advent2_enum() {
        for (input, output) in advent2_input() {
            assert_eq!(Ok(output.to_string()), advent2_enum::run(input))
        }
    }

    #[test]
    fn advent2_error() {
        for (input, output) in advent2_input() {
            assert_eq!(Ok(output.to_string()), advent2_error::run(input))
        }
    }

    #[test]
    fn advent2_thiserror() {
        for (input, output) in advent2_input() {
            assert_eq!(Ok(output.to_string()), advent2_thiserror::run(input))
        }
    }

    fn advent1_input() -> &'static [(u32, f32)] {
        &[(12, 2.), (14, 2.), (1969, 654.), (100_756, 33583.)]
    }

    fn advent2_input() -> &'static [(&'static str, &'static str)] {
        &[
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ]
    }
}
