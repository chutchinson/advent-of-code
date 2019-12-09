#[macro_use]
extern crate lazy_static;

mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let days: Vec<fn()> = vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve
    ];
    for (index, solve) in days.iter().enumerate() {
        println!("day {}", index + 1);
        println!("---");
        solve();
        println!("---");
    }
}

#[cfg(test)]
mod tests {
    fn extract_digit(value: i32, index: usize) -> i32 {
        let power = 10i32.pow(index as u32);
        let digit = (value / power) % 10;
        digit
    }
    #[test]
    fn example_1() {
        let value = 1234;
        let x = extract_digit(value, 3);
        assert_eq!(x, 1);
    }
}