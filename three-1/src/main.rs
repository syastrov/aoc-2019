use std::collections::HashMap;
use std::collections::HashSet;


type Value = i32;

const RIGHT: &str = "R";
const LEFT: &str = "L";
const UP: &str = "U";
const DOWN: &str = "D";
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

enum Direction {
    RIGHT, LEFT, UP, DOWN
}

struct Movement {
    direction: Direction,
    amount: i32,
}

struct Wire {
  movements: Vec<Movement>,
  id: u8,
}


impl Wire {
  fn new(input: String, id: u8) -> Self {
    Wire { 
        id: id,
        movements: input.split(",").map(
          |x: &str| {
              let first_char = &x[0..1];
              let direction = match first_char {
                  RIGHT => Direction::RIGHT,
                  LEFT => Direction::LEFT,
                  UP => Direction::UP,
                  DOWN => Direction::DOWN,
                  _ => {
                      panic!("Unknown direction: {}", first_char);
                  }
              };
              let amount = x[1..].parse::<Value>().unwrap();
              Movement { direction: direction, amount: amount }
          }
        ).collect()
    }
  }
}

struct Grid {
    cells: HashMap<(Value, Value), HashSet<u8>>
}

impl Grid {
    fn new() -> Self { 
      Grid {
        cells: HashMap::new()
      }
    }
    fn set(&mut self, x: Value, y: Value, wire_id: u8) {
        let cell = self.cells.entry((x, y)).or_insert_with(|| HashSet::new());
        let cell_value = &mut*cell;
        cell_value.insert(wire_id);
    }
  fn fill_grid(&mut self, wire: Wire) {
    let mut cur_x: Value= 0;
    let mut cur_y: Value = 0;
    for Movement { direction, amount } in wire.movements {
      match direction {
        Direction::UP => {
          for y in cur_y..cur_y + amount {
            self.set(cur_x, y, wire.id);
          }
          cur_y += amount;
        },
        Direction::DOWN => {
          //println!("Down {} from {}", amount, cur_y);
          for y in cur_y..cur_y - amount {
            self.set(cur_x, y, wire.id);
          }
          cur_y -= amount;
        },
        Direction::LEFT => {
          for x in cur_x..cur_x - amount {
            self.set(x, cur_y, wire.id);
          }
          cur_x -= amount;
        },
        Direction::RIGHT => {
          for x in cur_x..cur_x + amount {
            self.set(x, cur_y, wire.id);
          }
          cur_x += amount;
        },
      }
    }
  }


  fn get_intersections(self) -> Vec<(Value, Value)> {
      let mut intersections = vec![];
      for (point, wire_ids) in self.cells.iter() {
        if wire_ids.len() > 1 {
          intersections.push(point.clone());
        }
      }
      intersections
  }
}


fn main() {
  let wire1_str: String = "R997,U849,R349,U641,R581,D39,R285,U139,R455,D346,L965,D707,R393,D302,L263,U58,R950,U731,R858,D748,R302,U211,R588,D441,L153,D417,R861,U775,R777,U204,R929,U868,L62,U163,R841,D214,L648,U626,R501,D751,L641,D961,L23,D430,L73,D692,R49,U334,L601,U996,R444,D658,R633,D30,L861,D811,R10,D394,R9,U227,L848,U420,L378,D622,L501,U397,R855,U369,R615,U591,L674,D166,L181,U61,L224,U463,L203,U594,R93,U614,L959,U198,L689,D229,L674,U255,R843,D382,R538,U923,L960,D775,L879,U97,R137,U665,L340,D941,L775,D57,R852,D167,R980,U704,L843,U989,L611,D32,L724,D790,L32,U984,L39,U671,L994,U399,R475,D85,L322,D936,R117,D261,R705,D696,L523,D433,L239,U477,L247,D465,R560,D902,L589,U682,R645,U376,L989,D121,L215,U514,R519,U407,L218,D444,R704,D436,L680,U759,R937,U400,R533,D860,R782,D233,R840,D549,L508,U380,L992,U406,L213,D403,L413,D532,L429,U186,R262,U313,L913,U873,L838,D882,R851,U70,R185,D131,R945,D595,L330,U446,R88,D243,L561,D952,R982,D395,L708,U459,L82,D885,L996,U955,L406,U697,L183,U266,L878,D839,R843,D891,R118,U772,R590,D376,L500,U370,R607,D12,L310,D436,L602,D365,R886,U239,L471,D418,L122,U18,R879,D693,R856,U848,L657,D911,L63,U431,R41,U752,R919,U323,L61,D263,L370,D85,R929,D213,R350,U818,R458,D912,R509,U394,L734,U49,R810,D87,L870,U658,R499,U550,L402,U244,L112,U859,R836,U951,R222,D944,L691,D731,R742,D52,R984,D453,L514,U692,R812,U35,L844,D177,L110,D22,R61,U253,R618,D51,R163,U835,R704,U148,R766,U297,R457,D170,L104,D441,R330,D330,R989,D538,R668,D811,R62,D67,L470,D526,R788,U376,R708,U3,R961".to_string();
  let wire2_str: String = "L1009,D381,R970,U429,L230,D909,R516,D957,R981,U609,L480,D139,L861,U168,L48,U620,R531,D466,L726,D380,R977,D454,L318,D397,R994,U402,L77,U93,L359,D72,R968,D956,L174,D22,R218,U619,R593,U32,L154,U55,L169,U415,L171,U666,R617,U109,L265,U773,R541,D808,L797,U478,R731,U379,R311,D137,L806,U298,R362,D458,L254,D539,R700,U853,R246,D588,L28,U203,L432,U946,R663,D408,R974,U59,L683,D36,L139,U738,L780,U414,L401,D93,R212,D973,L710,U892,L357,D177,R823,D4,R46,D924,L235,D898,R67,U220,L592,U87,R94,U584,R979,D843,L299,D648,L491,U360,R824,D245,L944,D24,R616,U975,L4,U42,L984,U181,R902,D835,L687,D413,L767,U632,L754,U270,R413,U51,L825,D377,L596,U960,L378,U706,L859,D708,L156,D991,L814,U351,R923,D749,L16,D651,R20,D86,R801,U811,L228,U161,L871,U129,R215,U235,L784,U896,R94,U145,R822,U494,R248,D98,R494,U156,L495,U311,R66,D873,L294,D620,L885,U395,R778,D227,R966,U756,L694,D707,R983,D950,R706,D730,R415,U886,L465,D622,L13,D938,R324,D464,R723,U804,R942,D635,L729,D317,L522,U469,R550,D141,R302,U999,L642,U509,R431,D380,R18,D676,R449,D759,L495,U901,R1,D745,L655,U449,L439,D818,R55,D541,R420,U764,L426,D176,L520,U3,L663,D221,L80,D449,L987,U349,L71,U632,L887,D231,R655,D208,R698,D639,R804,U616,R532,U846,R363,D141,R659,U470,L798,U144,L675,U483,L944,U380,L329,U72,L894,D130,R53,U109,R610,U770,R778,U493,R972,D340,L866,U980,L305,D812,R130,D954,R253,D33,L912,U950,L438,D680,R891,U725,R171,D587,R549,D367,L4,U313,R522,D128,L711,D405,L769,D496,L527,U373,R725,D261,L268,D939,L902,D58,L858,D190,L442".to_string();
  let wire1 = Wire::new(wire1_str, 1);
  let wire2 = Wire::new(wire2_str, 2);

  let mut grid = Grid::new();

  grid.fill_grid(wire1);
  grid.fill_grid(wire2);

  let intersections = grid.get_intersections();
  let mut distances = intersections.iter().map(|(x, y)| x.abs() + y.abs()).collect::<Vec<Value>>();
  distances.sort();
  println!("{:?}", distances[0]);
  // TODO: Find minimum dist to bottom left
}
