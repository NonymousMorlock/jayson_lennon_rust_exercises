// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod clamp_test {
    use crate::clamp;

    #[test]
    fn clamps_to_lower_bound() {
        let t_number: i32 = 3;
        let t_lower: i32 = 5;
        let t_upper: i32 = 10;

        let result: i32 = clamp(t_number, t_lower, t_upper);

        assert_eq!(result, t_lower, "Expected {}; Actual{}", t_lower, result);
    }

    #[test]
    fn clamps_to_upper_bound() {
        let t_number: i32 = 12;
        let t_lower: i32 = 5;
        let t_upper: i32 = 10;

        let result: i32 = clamp(t_number, t_lower, t_upper);

        assert_eq!(result, t_upper, "Expected {}; Actual {}", t_upper, result);
    }

    #[test]
    fn retains_number_in_range() {
        let t_number: i32 = 8;
        let t_lower: i32 = 5;
        let t_upper: i32 = 10;

        let result: i32 = clamp(t_number, t_lower, t_upper);

        assert_eq!(result, t_number, "Expected: {}; Actual: {}", t_number, result);
    }
}

#[cfg(test)]
mod div_test {
    use crate::div;

    #[test]
    fn returns_the_correct_quotient() {
        let t_numerator: i32 = 12;
        let t_denominator: i32 = 24;
        let quotient: i32 = t_numerator / t_denominator;

        let result: Option<i32> = div(t_numerator, t_denominator);

        assert_eq!(result, Some(quotient), "Expected: {:?}; Actual: {:?}", Some(quotient), result);
    }

    #[test]
    fn uses_safe_division() {
        let t_numerator: i32 = 10;
        let t_denominator: i32 = 0;

        let result: Option<i32> = div(t_numerator, t_denominator);

        assert_eq!(result, None, "Expected: None; Actual: {:?}", result);
    }
}

#[cfg(test)]
mod concat_test {
    use crate::concat;

    #[test]
    fn concatenates_in_correct_order() {
        let t_first: &str = "first-";
        let t_second: &str = "second";
        let t_expected: String = format!("{}{}", t_first, t_second);

        let result: String = concat(t_first, t_second);

        assert_eq!(result, t_expected, "Expected: {:?}; Actual: {:?}", t_expected, result);
    }
}
