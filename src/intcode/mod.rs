#[derive(Debug)]
pub struct Intcode {
    pub halted: bool,
    pub memory: Vec<i32>,
    pub pc: usize,
    pub input: i32,
    pub outputs: Vec<i32>
}

const OP_HALT: i32 = 99;
const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;

fn extract_digit(value: i32, index: usize) -> i32 {
    let power = 10i32.pow(index as u32);
    let digit = (value / power) % 10;
    digit as i32
}

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Position,
    Immediate
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    op1: Parameter,
    op2: Parameter,
    op3: Parameter
}

impl From<i32> for Parameter {
    fn from(value: i32) -> Parameter {
        match value {
            1 => Parameter::Immediate,
            _ => Parameter::Position
        }
    }
}

impl Intcode {

    pub fn new() -> Intcode {
        Intcode {
            memory: Vec::new(),
            outputs: Vec::new(),
            pc: 0,
            halted: false,
            input: 0
        }
    }

    pub fn with_input(input: &str) -> Intcode {
        let program = Self::compile(input);
        let mut vm = Intcode::new();
        vm.reset(program);
        vm
    }

    pub fn compile(input: &str) -> Vec<i32> {
        input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    fn decode(&mut self) -> Instruction {
        let op = self.load();
        let opcode = op % 10;
        let (p1, p2, p3) = (extract_digit(op, 2), extract_digit(op, 3), extract_digit(op, 4));
        Instruction {
            opcode: opcode,
            op1: p1.into(),
            op2: p2.into(),
            op3: p3.into()
        }
    }

    pub fn reset(&mut self, memory: Vec<i32>) {
        self.memory = memory;
        self.pc = 0;
        self.halted = false;
        self.input = 0;
    }

    pub fn cycle(&mut self) {
        if self.halted {
            return
        }
        let op = self.decode();
        match op.opcode {
            OP_ADD => self.op_add(&op),
            OP_MUL => self.op_mul(&op),
            OP_HALT => self.op_halt(&op),
            OP_INPUT => self.op_input(&op),
            OP_OUTPUT => self.op_output(&op),
            _ => self.halted = true
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.cycle()
        }
    }

    pub fn read(&self, addr: i32) -> i32 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: i32, value: i32) {
        self.memory[addr as usize] = value
    }

    fn fetch(&mut self, parameter: Parameter) -> i32 {
        match parameter {
            Parameter::Immediate => self.load(),
            Parameter::Position => {
                let addr = self.load();
                self.read(addr)
            }
        }
    }

    fn load(&mut self) -> i32 {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }

    fn op_input(&mut self, _: &Instruction) {
        let dest = self.load();
        self.write(dest, self.input);
    }

    fn op_output(&mut self, op: &Instruction) {
        let value = self.fetch(op.op1);
        self.outputs.push(value);
    }
    
    fn op_halt(&mut self, _: &Instruction) {
        self.halted = true;
    }

    fn op_mul(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.load();
        self.write(dest, a * b);
    }

    fn op_add(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.load();
        self.write(dest, a + b);
    }

}