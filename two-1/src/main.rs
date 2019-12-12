type Value = u32;

const OPCODE_ADD: Value = 1;
const OPCODE_MULTIPLY: Value = 2;
const OPCODE_HALT: Value = 99;
// enum Opcode {
//   Add,
//   Multiply,
//   Halt,
// }

// impl Opcode {
//   fn from_value(n: Value) -> Option<Opcode> {
//       match n {
//           OPCODE_ADD => Some(Opcode::ADD),
//           OPCODE_MULTIPLY => Some(Opcode::MULTIPLY),
//           OPCODE_HALT => Some(Opcode::HALT),
//           _ => None,
//       }
//   }
// }

struct Program {
  data: Vec<Value>
}


trait ReadableWritable {
  fn get(&self, pos: Value) -> Value;
  fn set(&mut self, pos: Value, val: Value);
  fn get_all(&self) -> &Vec<Value>;
}

impl ReadableWritable for Program {
  fn get(&self, pos: Value) -> Value {
    let val = self.data[pos as usize];
    println!("Read {} from {}", val, pos);
    val
  }
  fn set(&mut self, pos: Value, val: Value) {
    let ptr = &mut self.data[pos as usize];
    *ptr = val;
    println!("Set {} to {}", pos, val);
  }
  fn get_all(&self) -> &Vec<Value> {
    &self.data
  }
}

fn main() {
  let intcode: String = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,2,9,19,23,2,13,23,27,1,6,27,31,2,6,31,35,2,13,35,39,1,39,10,43,2,43,13,47,1,9,47,51,1,51,13,55,1,55,13,59,2,59,13,63,1,63,6,67,2,6,67,71,1,5,71,75,2,6,75,79,1,5,79,83,2,83,6,87,1,5,87,91,1,6,91,95,2,95,6,99,1,5,99,103,1,6,103,107,1,107,2,111,1,111,5,0,99,2,14,0,0".to_string();
  let mut program = Program { 
    data: intcode.split(",").map(
      |x: &str| x.parse::<Value>().unwrap()
    ).collect()
  };
  let mut pos = 0;
  loop {
    let opcode = program.get(pos);
    println!("pos: {}, opcode: {}", pos, opcode);
    match opcode {
      OPCODE_ADD => {
        let a = program.get(program.get(pos+1));
        let b = program.get(program.get(pos+2));
        let c = program.get(pos+3);
        program.set(c, a + b);
        pos += 4;
      },
      OPCODE_MULTIPLY => {
        let a = program.get(program.get(pos+1));
        let b = program.get(program.get(pos+2));
        let c = program.get(pos+3);
        program.set(c, a * b);
        pos += 4;
      },
      OPCODE_HALT => {
        break;
      }
      _ => {
        println!("Invalid opcode: {}", opcode);
        return;
      },
    }
  }
  println!("Final state {:?}", program.get_all());
}
