mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Position = u32;
type Integer = i32;
type InputFn = Box<dyn FnMut(u32) -> Integer>;

struct Program {
    data: Vec<Integer>,
    input_fn: InputFn,
    input_call_count: u32
}

use num_enum::TryFromPrimitive;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum ParamMode {
    POSITION = 0,
    IMMEDIATE = 1,
}

const NUM_PARAMS: usize = 3;

type ParamModes = [ParamMode; NUM_PARAMS];

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum Instruction {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[wasm_bindgen]
pub fn run_program(code: &str) -> Integer {
    let mut program = Program::new(code, Box::new(|_| 1));
    program.execute().unwrap()
}

fn read_opcode(param: Integer) -> (Instruction, ParamModes) {
    let a = ((param / 10000) % 1000) as u8;
    let b = ((param / 1000) % 100) as u8;
    let c = ((param / 100) % 10) as u8;
//        TODO: Remove ugly use of unwrap. Should use ? to return Result's instead?
    let last_2_digits = (param % 100) as u8;
    let instruction = Instruction::try_from(last_2_digits).expect(format!("Last 2 digits of opcode {} are not a valid instruction", last_2_digits).as_str());
    let modes: ParamModes = [
//            Modes are in reverse order of the parameters :)
        ParamMode::try_from(c).unwrap(),
        ParamMode::try_from(b).unwrap(),
        ParamMode::try_from(a).unwrap(),
    ];
    (instruction, modes)
}

impl Program {
    pub fn new(intcode: &str, input_fn: InputFn) -> Self {
        let memory = intcode.split(",").map(
            |x: &str| x.parse::<Integer>().unwrap()
        ).collect();
        Program {
            data: memory,
            input_fn: input_fn,
            input_call_count: 0,
        }
    }
    fn get(&self, pos: Position) -> Integer {
        println!("\t\tRead from pos {}", pos);
        let val = self.data[pos as usize];
        println!("\t\tGot {}", val);
        val
    }
    fn read_param(&self, pos: Position, mode: &ParamMode) -> Integer {
        match mode {
            ParamMode::POSITION => self.get(self.get(pos) as Position),
            ParamMode::IMMEDIATE => self.get(pos)
        }
    }

    fn set(&mut self, pos: Position, val: Integer) {
        let ptr = &mut self.data[pos as usize];
        *ptr = val;
        println!("\t\tSet {} to {}", pos, val);
    }

    fn read_input(&mut self) -> Integer {
        self.input_call_count += 1;
        let result = (self.input_fn)(self.input_call_count);
        println!("Read input {}", result);
        result
    }
    fn write_output(&self, val: Integer) {
        println!("{}", val);
    }

    pub fn execute(self: &mut Program) -> Option<Integer> {
        self.input_call_count = 0;
        let mut pos = 0;
        let mut prev_output: Option<Integer> = None;
        loop {
            let opcode = self.get(pos);
            let (instruction, modes) = read_opcode(opcode);
            println!("\tpos: {}, instruction: {:?}, modes: {:?}", pos, instruction, modes);
            use Instruction::*;
            match instruction {
                Add => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.get(pos + 3);
                    self.set(c as Position, a + b);
                    pos += 4;
                }
                Multiply => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.get(pos + 3) as Position;
                    self.set(c, a * b);
                    pos += 4;
                }
                Input => {
                    let dest = self.get(pos + 1) as Position;
                    let input = self.read_input();
                    self.set(dest, input);
                    pos += 2;
                }
                Output => {
                    let val = self.read_param(pos + 1, &modes[0]);
                    self.write_output(val);
                    prev_output = Some(val);
                    pos += 2;
                }
                JumpIfTrue => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    if a != 0 {
                        pos = b as Position;
                    } else {
                        pos += 3;
                    }
                }
                JumpIfFalse => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    if a == 0 {
                        pos = b as Position;
                    } else {
                        pos += 3;
                    }
                }
                LessThan => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.get(pos + 3);
                    self.set(c.try_into().unwrap(), (a < b).try_into().unwrap());
                    pos += 4;
                }
                Equals => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.get(pos + 3);
                    self.set(c.try_into().unwrap(), (a == b).try_into().unwrap());
                    pos += 4;
                }
                Halt => {
                    break;
                }
            }
        }
        prev_output
    }
}
