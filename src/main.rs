mod day1;

fn main() {
    let days = vec![
        day1::solve
    ];
    for (index, solve) in days.iter().enumerate() {
        println!("day {}", index + 1);
        println!("---");
        solve();
        println!("---");
    }
}