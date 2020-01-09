use seven2::*;
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    use permutohedron::LexicalPermutation;

    let intcode_program = "3,8,1001,8,10,8,105,1,0,0,21,34,51,76,101,114,195,276,357,438,99999,3,9,1001,9,3,9,1002,9,3,9,4,9,99,3,9,101,4,9,9,102,4,9,9,1001,9,5,9,4,9,99,3,9,1002,9,4,9,101,3,9,9,102,5,9,9,1001,9,2,9,1002,9,2,9,4,9,99,3,9,1001,9,3,9,102,2,9,9,101,4,9,9,102,3,9,9,101,2,9,9,4,9,99,3,9,102,2,9,9,101,4,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,99";

    let mut possible_phases = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();

    loop {
        permutations.push(possible_phases.to_vec());
        if !possible_phases.next_permutation() {
            break;
        }
    }
    let max_output = permutations.iter().map(|phase_settings| {
        let mut rxs = VecDeque::new();
        let mut txs = VecDeque::new();
        let num_programs = phase_settings.len();
        for _ in 0..=num_programs {
            let (tx, rx) = channel();
            rxs.push_back(rx);
            txs.push_back(tx);
        }

        for (i, &phase) in phase_settings.iter().enumerate() {
            txs[i].send(phase).expect(format!("Error sending to program {}", i).as_str());
        }
        txs[0].send(0).expect("Error sending to first program");

        let threads: Vec<JoinHandle<Integer>> =
            (0..num_programs).map(|i| {
                let rx = rxs.pop_front().unwrap();
                let tx = if i == num_programs - 1 {
                    txs.pop_front().unwrap()
                } else {
                    txs.remove(1).unwrap()
                };
                thread::spawn(move || {
                    let input_fn = || {
                        let result = rx.recv();
                        let input = result.expect(format!("Channel hung up on program {}", i).as_str());
                        input
                    };
                    let output_fn = |output| {
                        // The last output will fail since the receiver won't be listening anymore.
                        // Therefore, ignore send errors.
                        let _ = tx.send(output);
                    };
                    let mut program = Program::new(intcode_program,
                                                   input_fn, output_fn);
                    let output = program.execute().expect("Program didn't output anything");
                    output
                })
            }).collect();

        let last_output = threads.into_iter().map(|thread| {
            let output = thread.join();
            output.unwrap()
        }).last();
        last_output.unwrap()
    }).max().unwrap();
    println!("Max output: {:?}.", max_output)
}
