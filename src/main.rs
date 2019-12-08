mod intcode;
mod day1;
mod day2;

fn main() {
    let days: Vec<fn()> = vec![
        day1::solve,
        day2::solve
    ];
    for (index, solve) in days.iter().enumerate() {
        println!("day {}", index + 1);
        println!("---");
        solve();
        println!("---");
    }
}