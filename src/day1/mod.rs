use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn solve() {
    let input = include_str!("./input.txt");
    let masses = input.lines()
        .map(|x| x.parse::<f32>().unwrap());
    let fuel_requirement_sum = masses.clone().fold(0.0, |total, mass| total + fuel_required(mass));
    let fuel_requirement_total_sum = masses.clone().fold(0.0, |total, mass| total + total_fuel_required(mass));
    println!("{}", fuel_requirement_sum);
    println!("{}", fuel_requirement_total_sum);
}

fn fuel_required(mass: f32) -> f32 {
    (mass / 3.0).floor() - 2.0
}

fn total_fuel_required(mass: f32) -> f32 {
    let fuel = fuel_required(mass);
    if fuel <= 0.0 { return 0.0 };
    return fuel + total_fuel_required(fuel);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn correct_total_fuel_requirement_for_given_mass() {
        assert_eq!(2.0, total_fuel_required(14.0));
        assert_eq!(966.0, total_fuel_required(1969.0));
        assert_eq!(50346.0, total_fuel_required(100756.0));
    }

    #[test]
    fn correct_fuel_requirement_for_given_mass() {
        assert_eq!(2.0, fuel_required(12.0));
        assert_eq!(2.0, fuel_required(14.0));
        assert_eq!(654.0, fuel_required(1969.0));
        assert_eq!(33583.0, fuel_required(100756.0));
    }

}