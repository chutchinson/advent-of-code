use crate::intcode::Intcode;

pub fn solve() {
    let input = include_str!("./input.txt");
    let program = Intcode::compile(input);
    let mut vm = Intcode::new();

    vm.reset(program.clone());
    vm.write(1, 12);
    vm.write(2, 2);
    vm.run();

    println!("{}", vm.read(0));

    for noun in 0..100 {
        for verb in 0..100 {
            vm.reset(program.clone());
            vm.write(1, noun);
            vm.write(2, verb);
            vm.run();

            if vm.read(0) == 19690720 {
                let answer = 100 * noun + verb;
                println!("{}", answer);
                return;
            }
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_state(program: &str, expected_state: Vec<i32>) {
        let mut vm = Intcode::new();
        let program = Intcode::compile(program);
        vm.reset(program);
        vm.run();
        // let state = &vm.memory[0..expected_state.len()];
        // assert_eq!(&expected_state[0..], state);
    }

    #[test]
    fn day_2_example_1() {
        assert_state("1,0,0,0,99", vec![2,0,0,0,99]);
        assert_state("2,3,0,3,99", vec![2,3,0,6,99]);
        assert_state("2,4,4,5,99,0", vec![2,4,4,5,99,9801]);
    }

}