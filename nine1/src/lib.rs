use num_enum::TryFromPrimitive;
use std::convert::{TryFrom, TryInto};
use log;

type Position = u64;
pub type Integer = i64;

pub struct Program<I, O>
    where I: Fn() -> Integer, O: Fn(Integer)
{
    data: Vec<Integer>,
    input_fn: I,
    output_fn: O,
    relative_base: Integer,
}


#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
enum ParamMode {
    POSITION = 0,
    IMMEDIATE = 1,
    RELATIVE = 2,
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
    RelativeBaseOffset = 9,
    Halt = 99,
}

fn read_opcode(param: Integer) -> (Instruction, ParamModes) {
    let a = ((param / 10000) % 1000) as u8;
    let b = ((param - (a as i64 * 10000)) / 1000) as u8;
    let c = ((param - (a as i64 * 10000) - (b as i64 * 1000)) / 100) as u8;
//        TODO: Remove ugly use of unwrap. Should use ? to return Result's instead?
    let last_2_digits = (param % 100) as u8;
    let instruction = Instruction::try_from(last_2_digits).expect(format!("Last 2 digits of opcode {} are not a valid instruction", last_2_digits).as_str());
    println!("opcode: {}. Modes a {} b {} c {}", param, a, b, c);
    let modes: ParamModes = [
//            Modes are in reverse order of the parameters :)
        ParamMode::try_from(c).unwrap(),
        ParamMode::try_from(b).unwrap(),
        ParamMode::try_from(a).unwrap(),
    ];
    (instruction, modes)
}

impl<I, O> Program<I, O>
    where I: Fn() -> Integer, O: Fn(Integer)
{
    pub fn new(intcode: &str, input_fn: I, output_fn: O) -> Self {
        let mut memory: Vec<Integer> = intcode.split(",").map(
            |x: &str| x.parse::<Integer>().unwrap()
        ).collect();
        memory.resize(10000000, 0);
        Self {
            data: memory,
            input_fn,
            output_fn,
            relative_base: 0
        }
    }
    /// Get value stored at position
    fn get(&self, pos: Position) -> Integer {
//        println!("\t\tRead from pos {}", pos);
        let val = self.data[pos as usize];
//        println!("\t\tGot {}", val);
        val
    }
    fn read_param(&self, pos: Position, mode: &ParamMode) -> Integer {
        let param_val = self.get(pos);
        match mode {
            ParamMode::POSITION => self.get(param_val.try_into().unwrap()),
            ParamMode::IMMEDIATE => param_val,
            ParamMode::RELATIVE => self.get((param_val + self.relative_base).try_into().unwrap()),
        }
    }

    fn read_output_param(&self, pos: Position, mode: &ParamMode) -> Position {
        let param_val = self.get(pos);
        let val = match mode {
            ParamMode::IMMEDIATE => panic!("Shouldn't pass param to Input opcode using IMMEDIATE param mode"),
            ParamMode::POSITION => param_val,
            ParamMode::RELATIVE => (param_val + self.relative_base)
        };
        val.try_into().unwrap()
    }

    fn set(&mut self, pos: Position, val: Integer) {
        let ptr = &mut self.data[pos as usize];
        *ptr = val;
        println!("\t\tSet {} to {}", pos, val);
    }

    fn read_input(&self) -> Integer {
        let result = (self.input_fn)();
        println!("Read input {}", result);
        result
    }

    fn write_output(&self, val: Integer) {
        println!("Outputting {}", val);
        (self.output_fn)(val);
    }

    pub fn execute(self: &mut Program<I, O>) -> Option<Integer> {
        let mut pos = 0;
        let mut prev_output: Option<Integer> = None;
        self.relative_base = 0;
        loop {
            let opcode = self.get(pos);
            let (instruction, modes) = read_opcode(opcode);
            println!("\tpos: {}, opcode: {}, instruction: {:?}, modes: {:?}", pos, opcode, instruction, modes);
            use Instruction::*;
            match instruction {
                Add => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.read_output_param(pos + 3, &modes[2]);
                    self.set(c, a + b);
                    pos += 4;
                }
                Multiply => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.read_output_param(pos + 3, &modes[2]);
                    self.set(c, a * b);
                    pos += 4;
                }
                Input => {
                    let dest = self.read_output_param(pos + 1, &modes[0]);
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
                    let c = self.read_output_param(pos + 3, &modes[2]);
                    self.set(c, (a < b).try_into().unwrap());
                    pos += 4;
                }
                Equals => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    let b = self.read_param(pos + 2, &modes[1]);
                    let c = self.read_output_param(pos + 3, &modes[2]);
                    self.set(c, (a == b).try_into().unwrap());
                    pos += 4;
                }
                RelativeBaseOffset => {
                    let a = self.read_param(pos + 1, &modes[0]);
                    self.relative_base += a;
                    println!("Relative base is now {}", self.relative_base);
                    pos += 2;
                }
                Halt => {
                    break;
                }
            }
        }
        prev_output
    }
}
