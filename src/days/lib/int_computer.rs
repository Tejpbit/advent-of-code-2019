use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use log::{info, trace, warn};

pub fn read_with_mode(arg: i32, memory: &Vec<i32>, mode: usize) -> i32 {
    if mode == 0 {
        memory[arg as usize]
    } else if mode == 1 {
        arg
    } else {
        panic!("Invalid read mode");
    }
}

pub fn run_computer(memory: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    let mut inputs = input.iter();
    let mut memory = memory;
    let mut outputs = vec![];
    let mut i = 0;
    loop {
        let next = memory[i];
        if next == 99 {
            break;
        }
        if next % 100 == 1 {
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;

            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let res_ref = memory[i + 3] as usize;
            memory[res_ref] = arg1 + arg2;
            i += 4;
        } else if next % 100 == 2 {
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let res_ref = memory[i + 3] as usize;
            memory[res_ref] = arg1 * arg2;
            i += 4;
        } else if next % 100 == 3 {
            let command_input: i32 = *inputs.next().unwrap();
            let res_pos = memory[i + 1] as usize;
            memory[res_pos] = command_input;
            i += 2;
        } else if next % 100 == 4 {
            let param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            let arg = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            outputs.push(arg);
            i += 2;
        } else if next % 100 == 5 {
            //        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.

            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);

            if arg1 != 0 {
                i = arg2 as usize;
            } else {
                i += 3;
            }
        } else if next % 100 == 6 {
            //        Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);

            if arg1 == 0 {
                i = arg2 as usize;
            } else {
                i += 3;
            }
        } else if next % 100 == 7 {
            //        Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let arg3 = memory[i + 3] as usize;

            memory[arg3] = if arg1 < arg2 { 1 } else { 0 };
            i += 4;
        } else if next % 100 == 8 {
            //        Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let arg3 = memory[i + 3] as usize;

            memory[arg3] = if arg1 == arg2 { 1 } else { 0 };
            i += 4;
        } else {
            panic!("Invalid operation")
        }
    }

    return outputs;
}

pub fn run_computer_channels(
    name: &'static str,
    memory: Vec<i32>,
    input: Receiver<Option<i32>>,
    output: Sender<Option<i32>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut memory = memory;
        let mut i = 0;
        loop {
            let next = memory[i];
            trace!("Machine {} instruction {}", name, next);
            if next == 99 {
                output.send(None);
                break;
            }
            if next % 100 == 1 {
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;

                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
                let res_ref = memory[i + 3] as usize;
                memory[res_ref] = arg1 + arg2;
                i += 4;
            } else if next % 100 == 2 {
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
                let res_ref = memory[i + 3] as usize;
                memory[res_ref] = arg1 * arg2;
                i += 4;
            } else if next % 100 == 3 {
                trace!("Machine {} reading input", name);
                let command_input: Option<i32> = match input.recv() {
                    Ok(input) => input,
                    Err(e) => panic!("Couldn't read from channel {}", e),
                };
                trace!("Machine {} read {}", name, command_input.unwrap());

                if command_input.is_none() {
                    trace!("Machine {} Inputter has quit, imma quit as well", name);
                    output.send(None);
                }

                let res_pos = memory[i + 1] as usize;
                memory[res_pos] = command_input.unwrap();
                i += 2;
            } else if next % 100 == 4 {
                let param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                let arg = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                trace!("Machine {} sending {}", name, arg);
                output.send(Some(arg)).unwrap();
                i += 2;
            } else if next % 100 == 5 {
                //        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.

                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);

                if arg1 != 0 {
                    i = arg2 as usize;
                } else {
                    i += 3;
                }
            } else if next % 100 == 6 {
                //        Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);

                if arg1 == 0 {
                    i = arg2 as usize;
                } else {
                    i += 3;
                }
            } else if next % 100 == 7 {
                //        Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
                let arg3 = memory[i + 3] as usize;

                memory[arg3] = if arg1 < arg2 { 1 } else { 0 };
                i += 4;
            } else if next % 100 == 8 {
                //        Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(memory[i + 1], &memory, param_mode_arg_1 as usize);
                let arg2 = read_with_mode(memory[i + 2], &memory, param_mode_arg_2 as usize);
                let arg3 = memory[i + 3] as usize;

                memory[arg3] = if arg1 == arg2 { 1 } else { 0 };
                i += 4;
            } else {
                panic!("Invalid operation")
            }
        }
    })
}
