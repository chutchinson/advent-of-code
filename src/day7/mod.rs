use crate::intcode::{Intcode, IntcodeBuilder};

pub fn solve() {
    let input = include_str!("./input.txt");

    {
        let phases = vec![0, 1, 2, 3, 4];
        let max_thruster_signal = permutations(phases)
            .map(|sequence| amplify_thruster_signal(input, &sequence, false))
            .max()
            .unwrap();

        println!("{}", max_thruster_signal);
    }

    {
        let phases = vec![5, 6, 7, 8, 9];
        let max_thruster_signal = permutations(phases)
            .map(|sequence| amplify_thruster_signal(input, &sequence, true))
            .max()
            .unwrap();

        println!("{}", max_thruster_signal);
    }

}

struct Permutations {
    array: Vec<i64>,
    swaps: Vec<usize>,
    index: usize
}

impl Iterator for Permutations {
    type Item = Vec<i64>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            loop {
                if self.index >= self.swaps.len() { return None; }
                if self.swaps[self.index] < self.index { break; }
                self.swaps[self.index] = 0;
                self.index += 1;
            }
            self.array.swap(self.index, (self.index & 1) * self.swaps[self.index]);
            self.swaps[self.index] += 1;
        }
        self.index = 1;
        Some(self.array.clone())
    }
}

fn permutations(array: Vec<i64>) -> Permutations {
    let n = array.len();
    let index = 0;
    Permutations {
        swaps: vec![0; n],
        array,
        index
    }
}

struct Amplifier {
    computer: Intcode
}

impl Amplifier {

    pub fn new(rom: &str, phase: i64) -> Amplifier {
        let mut computer = IntcodeBuilder::new()
            .with_program(rom)
            .with_inputs(&[phase])
            .build();
        Amplifier {
            computer
        }
    }

    pub fn run(&mut self, signal: i64) -> Option<i64> {
        self.computer.inputs.push_back(signal);
        self.computer.run_yield();
        self.computer.outputs.pop()
    }

}

fn amplify_thruster_signal(rom: &str, sequence: &[i64], feedback: bool) -> i64 {
    let mut amps: Vec<Amplifier> = sequence.iter()
        .map(|phase| Amplifier::new(rom, *phase))
        .collect(); 
    let mut next_signal = Some(0);
    let mut signal = 0;
    while next_signal.is_some() {
        for amplifier in amps.iter_mut() {
            next_signal = amplifier.run(signal);
            signal = next_signal.unwrap_or(signal);
        }
        if !feedback {
            break;
        }
    }
    return signal;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phases = vec![4,3,2,1,0];
        assert_eq!(43210, amplify_thruster_signal(program, &phases, false));
    }

    #[test]
    fn example_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phases = vec![0,1,2,3,4];
        assert_eq!(54321, amplify_thruster_signal(program, &phases, false));
    }

    #[test]
    fn example_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phases = vec![1,0,4,3,2];
        assert_eq!(65210, amplify_thruster_signal(program, &phases, false));
    }

    #[test]
    fn example_4() {
        let program = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phases = vec![9,8,7,6,5];
        assert_eq!(139629729, amplify_thruster_signal(program, &phases, true));
    }

    #[test]
    fn example_5() {
        let program = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let phases = vec![9,7,8,5,6];
        assert_eq!(18216, amplify_thruster_signal(program, &phases, true));
    }

}