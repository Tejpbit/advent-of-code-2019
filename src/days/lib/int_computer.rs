use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;

use log::{debug};

pub fn read_with_mode_old(arg: i32, memory: &Vec<i32>, mode: usize) -> i32 {
    if mode == 0 {
        memory[arg as usize]
    } else if mode == 1 {
        arg
    } else {
        panic!("Invalid read mode");
    }
}

pub fn read_with_mode(arg: i128, memory: &HashMap<u128, i128>, mode: i128, relative_base: i128) -> i128 {
    if mode == 0 {
        // POSITION mode
        debug!("Reading position mode, {}", arg);

        let parameter_value = deref(arg as u128, &memory);
        deref(parameter_value as u128, memory)
    } else if mode == 1 {
        // Immediate mode
        arg
    } else if mode == 2 {
        // Relative mode
        debug!("Reading relative mode, {}", relative_base+arg);
        let parameter_value = deref(arg as u128, &memory);
        deref((relative_base+parameter_value) as u128, &memory)
    } else {
        panic!("Invalid read mode");
    }
}

pub fn write_with_mode(pos_arg: u128, value: i128, memory: &mut HashMap<u128, i128>, mode: i128, relative_base: i128) {
    let dereferenced_pos_arg = deref(pos_arg, memory);
    match mode {
        0 => {
            debug!("Writing position mode {} {}", dereferenced_pos_arg, value);
            memory.insert(dereferenced_pos_arg as u128, value)
        },
        1 => panic!("Can't write with immediate"),//memory.insert(pos_arg, value), // This never makes sense
        2 => {
            debug!("Writing relative mode {} {}", dereferenced_pos_arg + relative_base, value);
            memory.insert((dereferenced_pos_arg + relative_base) as u128, value)
        },
        _ => panic!("Invalid write mode")
    };
}

pub fn deref(pos: u128, memory: &HashMap<u128, i128>) -> i128 {
    *memory.get(&pos).unwrap_or(&0i128)
}

pub fn _run_computer(memory: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
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

            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let res_ref = memory[i + 3] as usize;
            memory[res_ref] = arg1 + arg2;
            i += 4;
        } else if next % 100 == 2 {
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
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
            let arg = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            outputs.push(arg);
            i += 2;
        } else if next % 100 == 5 {
            //        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.

            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);

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
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);

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
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
            let arg3 = memory[i + 3] as usize;

            memory[arg3] = if arg1 < arg2 { 1 } else { 0 };
            i += 4;
        } else if next % 100 == 8 {
            //        Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
            let mut param_modes = next / 100; // remove op code
            let param_mode_arg_1 = param_modes % 10;
            param_modes /= 10;
            let param_mode_arg_2 = param_modes % 10;
            let arg1 = read_with_mode_old(memory[i + 1], &memory, param_mode_arg_1 as usize);
            let arg2 = read_with_mode_old(memory[i + 2], &memory, param_mode_arg_2 as usize);
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
    memory: Vec<i128>,
    input: Receiver<i128>,
    output: Sender<i128>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let vec_memory = memory;
        let mut memory: HashMap<u128, i128> = HashMap::new();
        for (i, value) in vec_memory.iter().enumerate() {
            memory.insert(i as u128, *value);
        }


        let mut i: u128 = 0;
        let mut relative_base = 0;
        loop {
            let next = deref(i, &memory);
            let mut s: Vec<(&u128, &i128)> = memory.iter().collect();
            s.sort_by(|a,b| a.0.cmp(&b.0));
            let s: Vec<&i128> = s.iter().map(|(_x,y)| *y).collect();

            debug!("\n");
            debug!("Machine {} | pc {} | instruction {} | relative_base {}", name, i, next, relative_base);
            let mut row = 0;
            s.chunks(10).for_each(|x| {
                debug!("{:0>3}: {:?}", row*10, x);
                row += 1;
            });
            if next == 99 {
                debug!("Machine {} halting", name);
                break;
            }
            if next % 100 == 1 {
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;

                let arg1 = read_with_mode(deref(i+1, &memory), &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode(deref(i+2, &memory), &memory, param_mode_arg_2, relative_base);
                let res_ref = deref(i+3, &memory) as u128;
                debug!("Machine {} ADD {} {} -> {}", name, arg1, arg2, res_ref);
                memory.insert(res_ref, arg1 + arg2);
                i += 4;
            } else if next % 100 == 2 {
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_3 = param_modes % 10;
                let arg1 = read_with_mode(deref(i+1, &memory) as i128, &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode(deref(i+2, &memory) as i128, &memory, param_mode_arg_2, relative_base);
                debug!("Machine {} MUL {} {} -> {}", name, param_mode_arg_1, param_mode_arg_2, param_mode_arg_3);
                debug!("Machine {} MUL {} {} -> {}", name, arg1, arg2, i+3);
                write_with_mode(i+3, arg1*arg2, &mut memory, param_mode_arg_3, relative_base);
                i += 4;
            } else if next % 100 == 3 {
                debug!("Machine {} reading input", name);
                let command_input: i128 = match input.recv() {
                    Ok(input) => input,
                    Err(e) => {
                        debug!("Input channel has closed {:?}", e);
                        continue;
                    },
                };
                debug!("Machine {} read {}", name, command_input);
                let param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;



                let res_pos = deref(i+1, &memory) as u128;
                write_with_mode(res_pos, command_input, &mut memory, param_mode_arg_1, relative_base);
                i += 2;
            } else if next % 100 == 4 {
                let param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                let arg = read_with_mode(i as i128 + 1, &memory, param_mode_arg_1, relative_base);
                debug!("Machine {} sending {}", name, arg);
                output.send(arg).unwrap();
                i += 2;
            } else if next % 100 == 5 {
                //        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.

                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(i as i128+1, &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode(i as i128+2, &memory, param_mode_arg_2, relative_base);
                debug!("Machine {} JTR {} {}", name, arg1, arg2);

                if arg1 != 0 {
                    i = arg2 as u128;
                } else {
                    i += 3;
                }
            } else if next % 100 == 6 {
                //        Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(i as i128+1, &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode(i as i128+2, &memory, param_mode_arg_2, relative_base);
                debug!("Machine {} JFA {} {}", name, arg1, arg2);

                if arg1 == 0 {
                    i = arg2 as u128;
                } else {
                    i += 3;
                }
            } else if next % 100 == 7 {
                //        Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let mut param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                let arg1 = read_with_mode(i as i128+1, &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode(i as i128+2, &memory, param_mode_arg_2, relative_base);
                let arg3 = deref(i+3, &memory) as u128;
                debug!("Machine {} LES {} {} {}", name, arg1, arg2, arg3);

                memory.insert(arg3, if arg1 < arg2 { 1 } else { 0 });
                i += 4;
            } else if next % 100 == 8 {
                //        Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let mut param_modes = next / 100; // remove op code
                debug!("All param modes: {} ", param_modes);

                let param_mode_arg_1 = param_modes % 10;
                param_modes /= 10;
                let param_mode_arg_2 = param_modes % 10;
                debug!("Param modes: {} {}", param_mode_arg_1, param_mode_arg_2);
                let arg1 = read_with_mode((i+1) as i128, &memory, param_mode_arg_1, relative_base);
                let arg2 = read_with_mode((i+2) as i128, &memory, param_mode_arg_2, relative_base);
                let arg3 = deref(i+3, &memory) as u128;

                debug!("Machine {} EQU {} {} {}", name, arg1, arg2, arg3);

                memory.insert(arg3, if arg1 == arg2 { 1 } else { 0});
                i += 4;
            } else if next % 100 == 9 {

                //        Opcode 9 adjusts the relative base
                let param_modes = next / 100; // remove op code
                let param_mode_arg_1 = param_modes % 10;
                let arg1 = read_with_mode(i as i128+1, &memory, param_mode_arg_1, relative_base);
                debug!("Machine {} REL {}", name, arg1);

                relative_base += arg1;
                i += 2;
            } else {
                panic!("Invalid operation")
            }
        }
    })
}
