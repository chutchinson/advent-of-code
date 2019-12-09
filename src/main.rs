mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let days: Vec<fn()> = vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve
    ];
    for (index, solve) in days.iter().enumerate() {
        println!("day {}", index + 1);
        println!("---");
        solve();
        println!("---");
    }
}