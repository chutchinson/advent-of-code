pub struct Intcode {
    pub halted: bool,
    pub memory: Vec<i32>,
    pub pc: usize
}

const OP_HALT: i32 = 99;
const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;

impl Intcode {

    pub fn new() -> Intcode {
        Intcode {
            memory: Vec::new(),
            pc: 0,
            halted: false
        }
    }

    pub fn compile(input: &str) -> Vec<i32> {
        input
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    pub fn reset(&mut self, memory: Vec<i32>) {
        self.memory = memory;
        self.pc = 0;
        self.halted = false;
    }

    pub fn cycle(&mut self) {
        if self.halted {
            return
        }
        let op = self.load();
        match op {
            OP_ADD => self.add(),
            OP_MUL => self.mul(),
            OP_HALT => self.halt(),
            _ => self.halt()
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.cycle()
        }
    }

    fn load(&mut self) -> i32 {
        let value = self.memory[self.pc];
        self.pc += 1;
        value
    }

    fn fetch(&mut self) -> i32 {
        let addr = self.load();
        self.read(addr)
    }

    pub fn read(&self, addr: i32) -> i32 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: i32, value: i32) {
        self.memory[addr as usize] = value
    }
    
    fn halt(&mut self) {
        self.halted = true;
    }

    fn mul(&mut self) {
        let a = self.fetch();
        let b = self.fetch();
        let dest = self.load();
        self.write(dest, a * b);
    }

    fn add(&mut self, ) {
        let a = self.fetch();
        let b = self.fetch();
        let dest = self.load();
        self.write(dest, a + b);
    }

}