use crate::intcode::{IntcodeBuilder};

pub fn solve() {
    let input = include_str!("./input.txt");
    let execute = |value| {
        let mut vm = IntcodeBuilder::new()  
            .with_program(input)
            .with_inputs(&[value])
            .build();
        vm.run();
        *vm.outputs.last().unwrap()
    };

    let diagnostic_code_1 = execute(1);
    let diagnostic_code_2 = execute(5);

    println!("{}", diagnostic_code_1);
    println!("{}", diagnostic_code_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_memory(program: &str, input: i64, addr: i64, expected: i64) {
        let mut vm = IntcodeBuilder::new()
            .with_program(program)
            .with_inputs(&[input])
            .build();
        vm.run();
        let v = vm.read(addr);
        assert_eq!(expected, v);
    }

    fn assert_output(program: &str, input: i64, output: i64) {
        let mut vm = IntcodeBuilder::new()
            .with_program(program)
            .with_inputs(&[input])
            .build();
        vm.run();
        let value = vm.outputs.last().unwrap();
        assert_eq!(output, *value);
    }

    #[test]
    fn mul_imm() {
        let input = "1002,4,3,4,33";       
        assert_memory(input, 0, 4, 99);
    }

    #[test]
    fn ceq_pos() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_output(input, 8, 1);
        assert_output(input, 7, 0);
    }

    #[test]
    fn clt_pos() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_output(input, 7, 1);
        assert_output(input, 8, 0);
    }

    #[test]
    fn ceq_imm() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        assert_output(input, 8, 1);
        assert_output(input, 7, 0);
    }

    #[test]
    fn clt_imm() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        assert_output(input, 7, 1);
        assert_output(input, 8, 0);
    }

    #[test]
    fn jump_if_true() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_output(input, 100, 1);
        assert_output(input, 0, 0);
    }

}