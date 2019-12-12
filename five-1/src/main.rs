mod intcode {
    type Position = u32;
    type Integer = i32;
    type InputFn = fn() -> Integer;

    pub struct Program {
        data: Vec<Integer>,
        input_fn: InputFn,
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

    fn read_opcode(param: Integer) -> (Instruction, ParamModes) {
        let a = ((param / 10000) % 1000) as u8;
        let b = ((param / 1000) % 100) as u8;
        let c = ((param / 100) % 10) as u8;
//        TODO: Remove ugly use of unwrap. Should use ? to return Result's instead?
        let last_2_digits = (param % 100) as u8;
        let instruction = Instruction::try_from(last_2_digits).unwrap();
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

        fn read_input(&self) -> Integer {
            (self.input_fn)()
        }
        fn write_output(&self, val: Integer) {
            println!("{}", val);
        }

        pub fn execute(self: &mut Program) {
            let mut pos = 0;
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
        }
    }
}

fn main() {
    use intcode::Program;
//    let mut program = Program::new("3,0,4,0,99", || 1);
    let mut program = Program::new("3,225,1,225,6,6,1100,1,238,225,104,0,1102,67,92,225,1101,14,84,225,1002,217,69,224,101,-5175,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1,214,95,224,101,-127,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1101,8,41,225,2,17,91,224,1001,224,-518,224,4,224,1002,223,8,223,101,2,224,224,1,223,224,223,1101,37,27,225,1101,61,11,225,101,44,66,224,101,-85,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1102,7,32,224,101,-224,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1001,14,82,224,101,-174,224,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,102,65,210,224,101,-5525,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,81,9,224,101,-90,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,71,85,225,1102,61,66,225,1102,75,53,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,226,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,359,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,404,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,419,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,539,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,569,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,599,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,629,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,644,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,674,101,1,223,223,4,223,99,226",
                                   || 5);
    program.execute();
}
