use std::cmp::Ordering;
use std::ops::Range;

pub fn solve() {
    let range = 264360..746325+1;
    let password_count_1 = count_valid_passwords(range.clone(), is_valid_password_1);
    let password_count_2 = count_valid_passwords(range, is_valid_password_2);

    println!("{}", password_count_1);
    println!("{}", password_count_2);
}

fn count_valid_passwords<F>(range: Range<usize>, predicate: F) -> usize
    where F: Fn(usize) -> bool {
    range
        .filter(|x| predicate(*x))
        .count()
}

fn is_valid_password_1(input: usize) -> bool {
    return is_valid_password(input, false);
}

fn is_valid_password_2(input: usize) -> bool {
    return is_valid_password(input, true);
}

fn is_valid_password(input: usize, minimal_double: bool) -> bool {
    let digits: Vec<u32> = input.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let mut has_minimal_double = false;
    for window in digits.windows(2) {
        let a = window[0];
        let b = window[1];
        match a.cmp(&b) {
            Ordering::Less => (),
            Ordering::Greater => return false,
            Ordering::Equal => {
                has_minimal_double |= if minimal_double {
                    !digits.windows(3).any(|x| x[0] == a && x[1] == a && x[2] == a)
                }
                else {
                    true
                }
            }
        }
    }
    has_minimal_double
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn passwords_meet_criteria() {
        assert_eq!(true, is_valid_password_1(111111));
        assert_eq!(false, is_valid_password_1(223450));
        assert_eq!(false, is_valid_password_1(123789));
    }

    #[test]
    fn passwords_match_if_digits_not_part_of_larger_group() {
        assert_eq!(true, is_valid_password_2(112233));
        assert_eq!(false, is_valid_password_2(123444));
        assert_eq!(true, is_valid_password_2(111122));
    }

}
