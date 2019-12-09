use crate::intcode::Intcode;

pub fn solve() {

    let input = include_str!("./input.txt");
    let mut vm = Intcode::with_input(input);

    vm.input = 1;
    vm.run();

    let diagnostic_code = vm.outputs.last().unwrap();
    
    println!("{}", diagnostic_code);

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_1() {
        let input = "1002,4,3,4,33";       
        let mut vm = Intcode::with_input(input);
        vm.run();
        let v = vm.read(4);
        assert_eq!(v, 99);
    }

}