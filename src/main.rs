mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let days: Vec<fn()> = vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve
    ];
    for (index, solve) in days.iter().enumerate() {
        println!("day {}", index + 1);
        println!("---");
        solve();
        println!("---");
    }
}