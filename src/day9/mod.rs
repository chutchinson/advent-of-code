use crate::intcode::{Intcode, IntcodeBuilder};

pub fn solve() {
    
    let program = include_str!("./input.txt");
    let memory = Intcode::compile(program);

    let mut vm = IntcodeBuilder::new()
        .with_memory_size(4096)
        .with_memory(&memory)
        .with_inputs(&[1])
        .build();

    vm.run();

    let code = vm.outputs.pop().unwrap();
    println!("{:?}", code);

    let mut vm = IntcodeBuilder::new()
        .with_memory_size(4096)
        .with_memory(&memory)
        .with_inputs(&[2])
        .build();

    vm.run();

    let code = vm.outputs.pop().unwrap();
    println!("{:?}", code);

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn relative_base_is_computed() {
        let program = [109,19,204,-34,99];
        let mut vm = IntcodeBuilder::new()
            .with_relative_base(2000)
            .with_memory_size(4096)
            .with_memory(&program)
            .build();
        vm.memory[1985] = 0x1234;
        vm.run();
        assert_eq!(2019, vm.relative_base);
        assert_eq!(0x1234, vm.outputs.pop().unwrap());
    }

    #[test]
    fn outputs_quine() {
        let program = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut vm = IntcodeBuilder::new()
            .with_memory(&program)
            .with_memory_size(128)
            .build();
        vm.run();
        let x = vm.outputs.into_iter().collect::<Vec<_>>();
        assert_eq!(&program[0..], &x[0..]);
    }

    #[test]
    fn outputs_16digit_number() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let mut vm = IntcodeBuilder::new()
            .with_program(program)
            .build();
        vm.run();
        let output = vm.outputs.pop().unwrap();
        assert_eq!(16, output.to_string().len());
    }

    #[test]
    fn outputs_1125899906842624() {
        let program = "104,1125899906842624,99";
        let mut vm = IntcodeBuilder::new()
            .with_program(program)
            .build();
        vm.run();
        let output = vm.outputs.pop().unwrap();
        assert_eq!(1125899906842624, output);
    }

}