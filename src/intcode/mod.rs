use std::collections::VecDeque;

pub type IntcodeType = i64;

#[derive(Debug)]
pub struct Intcode {
    pub halted: bool,
    pub memory: Vec<IntcodeType>,
    pub pc: usize,
    pub inputs: VecDeque<IntcodeType>,
    pub outputs: Vec<IntcodeType>,
    pub yielding: bool,
    pub relative_base: i64
}

const OP_HALT: u8 = 99;
const OP_ADD: u8 = 1;
const OP_MUL: u8 = 2;
const OP_INPUT: u8 = 3;
const OP_OUTPUT: u8 = 4;
const OP_JT: u8 = 5;
const OP_JF: u8 = 6;
const OP_CLT: u8 = 7;
const OP_CEQ: u8 = 8;
const OP_RBO: u8 = 9;

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Position,
    Immediate,
    Relative
}

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    op1: Parameter,
    op2: Parameter,
    op3: Parameter
}

impl From<IntcodeType> for Parameter {
    fn from(value: IntcodeType) -> Parameter {
        match value {
            0 => Parameter::Position,
            1 => Parameter::Immediate,
            2 => Parameter::Relative,
            _ => panic!("invalid parameter mode: {}", value)
        }
    }
}

pub struct IntcodeBuilder {
    memory: Vec<IntcodeType>,
    inputs: Vec<IntcodeType>,
    relative_base: IntcodeType
}

impl IntcodeBuilder {

    pub fn new() -> IntcodeBuilder {
        IntcodeBuilder {
            memory: Vec::new(),
            inputs: Vec::new(),
            relative_base: 0
        }
    }

    pub fn with_memory(mut self, memory: &[IntcodeType]) -> IntcodeBuilder {
        let size = std::cmp::max(memory.len(), self.memory.len());
        self.memory.resize(size, 0);
        let slice = &mut self.memory[0..memory.len()];
        slice.copy_from_slice(memory);
        self
    }

    pub fn with_inputs(mut self, inputs: &[IntcodeType]) -> IntcodeBuilder {
        self.inputs.extend(inputs.iter());
        self
    }

    pub fn with_relative_base(mut self, value: IntcodeType) -> IntcodeBuilder {
        self.relative_base = value;
        self
    }

    pub fn with_program(mut self, input: &str) -> IntcodeBuilder {
        let program = Intcode::compile(input);
        self.with_memory(&program)
    }

    pub fn with_memory_size(mut self, size: usize) -> IntcodeBuilder {
        self.memory.resize(size, Default::default());
        self
    }

    pub fn build(self) -> Intcode {
        let mut vm = Intcode::new();
        vm.reset(self.memory);
        vm.relative_base = self.relative_base;
        vm.inputs.extend(self.inputs.iter());
        vm
    }

}

impl Intcode {

    pub fn new() -> Intcode {
        Intcode {
            memory: Vec::new(),
            inputs: VecDeque::new(),
            outputs: Vec::new(),
            pc: 0,
            halted: false,
            yielding: false,
            relative_base: 0
        }
    }

    pub fn compile(input: &str) -> Vec<IntcodeType> {
        input
            .split(",")
            .map(|x| x.parse::<IntcodeType>().unwrap())
            .collect()
    }

    fn decode(&mut self) -> Instruction {
        let op = self.load();
        let opcode = op % 100;
        let p1 = (op / 100) % 10;
        let p2 = (op / 1000) % 10;
        let p3 = (op / 10000) % 10;
        Instruction {
            opcode: opcode as u8,
            op1: p1.into(),
            op2: p2.into(),
            op3: p3.into()
        }
    }

    pub fn reset(&mut self, memory: Vec<IntcodeType>) {
        self.memory = memory;
        self.pc = 0;
        self.halted = false;
        self.yielding = false;
        self.inputs.clear();
        self.outputs.clear();
        self.relative_base = 0;
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
            OP_JT => self.op_jt(&op),
            OP_JF => self.op_jf(&op),
            OP_CEQ => self.op_ceq(&op),
            OP_CLT => self.op_clt(&op),
            OP_RBO => self.op_rbo(&op),
            _ => panic!("unknown opcode")
        }
    }

    pub fn run_yield(&mut self) {
        while !self.halted {
            self.yielding = false;
            self.cycle();
            if self.yielding {
                return;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.cycle();
        }
    }

    pub fn read(&self, addr: IntcodeType) -> IntcodeType {
        if addr < 0 {
            panic!("attempt to read from negative memory address");
        }
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: IntcodeType, value: IntcodeType) {
        self.memory[addr as usize] = value
    }

    fn fetch_out(&mut self, parameter: Parameter) -> IntcodeType {
        let addr = self.load();
        match parameter {
            Parameter::Position => addr,
            Parameter::Relative => self.relative_base + addr,
            _ => panic!("invalid parameter mode")
        }
    }

    fn fetch(&mut self, parameter: Parameter) -> IntcodeType {
        let addr = self.load();
        match parameter {
            Parameter::Immediate => addr,
            Parameter::Position => self.read(addr),
            Parameter::Relative => self.read(self.relative_base + addr)
        }
    }

    fn load(&mut self) -> IntcodeType {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }

    fn op_rbo(&mut self, op: &Instruction) {
        let offset = self.fetch(op.op1);
        self.relative_base += offset;
    }

    fn op_jt(&mut self, op: &Instruction) {
        let cond = self.fetch(op.op1);
        let addr = self.fetch(op.op2);
        if cond != 0 {
            self.pc = addr as usize;
        }
    }

    fn op_jf(&mut self, op: &Instruction) {
        let cond = self.fetch(op.op1);
        let addr = self.fetch(op.op2);
        if cond == 0 {
            self.pc = addr as usize;
        }
    }

    fn op_clt(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.fetch_out(op.op3);
        self.write(dest, if a < b { 1 } else { 0 });
    }

    fn op_ceq(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.fetch_out(op.op3);
        self.write(dest, if a == b { 1 } else { 0 });
    }

    fn op_input(&mut self, op: &Instruction) {
        let dest = self.fetch_out(op.op1);
        let value = self.inputs.pop_front().unwrap_or(0);
        self.write(dest, value);
    }

    fn op_output(&mut self, op: &Instruction) {
        let value = self.fetch(op.op1);
        self.outputs.push(value);
        self.yielding = true;
    }
    
    fn op_halt(&mut self, _: &Instruction) {
        self.halted = true;
    }

    fn op_mul(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.fetch_out(op.op3);
        self.write(dest, a * b);
    }

    fn op_add(&mut self, op: &Instruction) {
        let a = self.fetch(op.op1);
        let b = self.fetch(op.op2);
        let dest = self.fetch_out(op.op3);
        self.write(dest, a + b);
    }

}