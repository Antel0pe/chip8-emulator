use std::fs;

use crate::display::Display;
use crate::config::Configuration;

mod display;
mod config;

fn main() {
    /*
     * 80 - 159 = Font data
     * 160 - 162 = Index Register
     */
    let mut ram: [u8; 4096] = [0; 4096];

    let mut program_counter: u16 = 512; 
    // let mut index_register: u16 = 0;
    let mut stack: Vec<u16> = Vec::new(); // really should be part of main memory 
    let mut delay_timer: u8 = 0;
    let mut sound_timer: u8 = 0;
    let mut variable_registers: [u8; 16] = [0; 16];

    let file_path = "roms/IBM Logo.ch8";

    let content = fs::read(file_path)
        .expect(&format!("Could not read ROM: {}", file_path));

    dotenv::dotenv().expect("Could not read .env file.");
    let config = envy::from_env::<Configuration>()
        .expect("No environment variables were able to be loaded by envy.");
    println!("env {:?}", config);
    

    // println!("ROM Contents: {:?}", content);
    // println!("ROM Contents in hex: {:02X?}", content);

    // let instructions = content.chunks(2)
    //     .map(|b| format!("{:02X?}{:02X?}", b[0], b[1]).to_string())
    //     .collect::<Vec<String>>()
    //     .join(" ");
    // println!("ROM Instructions {}", instructions);

    // let hex_instructions = content.chunks(2)
    //     .map(|b| [b[0], b[1]])
    //     .map(|b| u16::from_be_bytes(b))
    //     .collect::<Vec<u16>>();
    // println!("Hex ROM Instructions {:?}", hex_instructions);

    // println!("1st instruction: {}", 0b1000000000000000 & hex_instructions[1]);
    // println!("2nd instruction: {}", 0b0000100000000000 & hex_instructions[1]);
    // println!("3rd instruction: {}", 0b0000000010000000 & hex_instructions[1]);
    // println!("4th instruction: {}", 0b0000000000001000 & hex_instructions[1]);

    // let nibble_instructions = content.chunks(2)
    //     .map(|b| (b[0] >> 4, b[0] & 0b00001111, b[1] >> 4, b[1] & 0b00001111))
    //     .fold(Vec::new(), |mut acc, e| {
    //         acc.push(e);
    //         acc
    //     });
    // println!("'U4' Instructions: {:?}", nibble_instructions);

    // load memory in to 512 in decimal
    ram[512..512+content.len()]
        .clone_from_slice(&content);
    
    load_fonts(&mut ram);

    // println!("RAM Contents: {:?}", ram);
    
    let mut display = Display::new();

    loop{
        let first_byte = ram.get(program_counter as usize).unwrap();
        let second_byte = ram.get((program_counter+1) as usize).unwrap();
        program_counter += 2;

        let current_instruction = (
            first_byte >> 4, first_byte & 0b00001111,
            second_byte >> 4, second_byte & 0b00001111,
        );      

        println!("{:?}", current_instruction);

        match current_instruction {
            // 00E0
            (0, 0, 0xE, 0) =>{
                display.clear_screen();
                println!("Clear screen!");
            },
            // 00EE
            (0, 0, 0xE, 0xE) =>{
                let return_point = stack.pop().unwrap();

                println!("Returning from subroutine to address {}", return_point);

                program_counter = return_point;
            }
            // 1NNN
            (1, n0, n1, n2) =>{
                // let address = u16::from_be_bytes([n0, (n1 << 4 | n2)]);
                let address = extract_12_bit_number(n0, n1, n2);
                println!("Jump to {}", address);

                program_counter = address;

                // println!("Remove if not working on IBM Logo");
                // ::std::thread::sleep(Duration::new(1000, 1));
                // break; // TEMP BECAUSE IBM LOGO REPEATS HERE
            },
            //2NNN
            (2, n0, n1, n2) =>{
                // let address = u16::from_be_bytes([n0, (n1 << 4 | n2)]);
                let address = extract_12_bit_number(n0, n1, n2);
                println!("Calling address {}", address);

                stack.push(program_counter);

                program_counter = address;
            }
            //3XNN
            (3, x, n0, n1) =>{
                let value = extract_8_bit_number(n0, n1);

                if variable_registers[x as usize] == value{
                    println!("Skipping an instruction since {} = {}", variable_registers[x as usize], value);
                    program_counter += 2;
                } else{
                    println!("Not skipping an instruction since {} != {}", variable_registers[x as usize], value);
                }
            }
            //4XNN
            (4, x, n0, n1) =>{
                let value = extract_8_bit_number(n0, n1);

                if variable_registers[x as usize] != value{
                    println!("Skipping an instruction since {} != {}", variable_registers[x as usize], value);
                    program_counter += 2;
                } else{
                    println!("Not skipping an instruction since {} = {}", variable_registers[x as usize], value);
                }
            }
            //5XY0
            (5, x, y, 0) =>{
                if variable_registers[x as usize] == variable_registers[y as usize]{
                    println!("Skipping an instruction since {} = {}", variable_registers[x as usize], variable_registers[y as usize]);
                    program_counter += 2;
                } else{
                    println!("Not skipping an instruction since {} != {}", variable_registers[x as usize], variable_registers[y as usize]);
                }
            }
            //6XNN
            (6, x, n0, n1) =>{
                let value = extract_8_bit_number(n0, n1);

                println!("Set register V{} to {}", x, value);

                variable_registers[x as usize] = value;
            },
            //7XNN
            (7, x, n0, n1) =>{
                let value = extract_8_bit_number(n0, n1);

                println!("To register V{} add {}", x, value);

                // is this alright if it overflows?
                variable_registers[x as usize] = variable_registers[x as usize].wrapping_add(value);
            },
            //8XY0
            (8, x, y, 0) =>{
                variable_registers[x as usize] = variable_registers[y as usize];
                println!("Register {} is set to register {} - value of {}", x, y, variable_registers[y as usize]);
            },
            //8XY1
            (8, x, y, 1) =>{
                variable_registers[x as usize] |= variable_registers[y as usize];
                println!("Register {} is OR'd with register {}", x, y);
            },
            //8XY2
            (8, x, y, 2) =>{
                variable_registers[x as usize] &= variable_registers[y as usize];
                println!("Register {} is AND'd with register {}", x, y);
            },
            //8XY3
            (8, x, y, 3) =>{
                variable_registers[x as usize] ^= variable_registers[y as usize];
                println!("Register {} is XOR'd with register {}", x, y);
            },
            //8XY4
            (8, x, y, 4) =>{
                let (sum, is_overflow) = variable_registers[x as usize].overflowing_add(variable_registers[y as usize]);
                
                variable_registers[x as usize] = sum;

                if is_overflow{
                    variable_registers[0xF] = 1;
                    println!("Adding register {} to {}. Overflowed, setting VF to 1.", x, y);
                } else{
                    variable_registers[0xF] = 0;
                    println!("Adding register {} to {}. No overflow, setting VF to 0.", x, y);
                }
            },
            //8XY5
            // POTENTIAL ERROR IN SUBTRACTION
            (8, x, y, 5) =>{
                variable_registers[x as usize] = variable_registers[x as usize].wrapping_sub(variable_registers[y as usize]);
                
                let is_underflow = variable_registers[x as usize] > (variable_registers[y as usize]);
                
                if is_underflow{
                    variable_registers[0xF] = 0;

                    println!("Subtracting register {} from {}. Underflowed, setting VF to 0.", x, y);
                } else{
                    variable_registers[0xF] = 1;
                    println!("Subtracting register {} from {}. No underflow, setting VF to 1.", x, y);
                }
            },
            //8XY6
            (8, x, y, 6) =>{
                if config.ignore_y_in_8xy_shift_instruction{
                    println!("Ignore Y in 8XY shift instruction");
                } else{
                    println!("Use Y in 8XY shift instruction");
                    variable_registers[x as usize] = variable_registers[y as usize];
                }

                let shifted_out_bit = variable_registers[x as usize] & 0b00000001; 
                variable_registers[x as usize] >>= 1;

                variable_registers[0xF] = shifted_out_bit & 1;
            },
            //8XY7
            (8, x, y, 7) =>{
                variable_registers[x as usize] = variable_registers[y as usize].wrapping_sub(variable_registers[x as usize]);
                
                let is_underflow = variable_registers[y as usize] > (variable_registers[x as usize]);
                
                if is_underflow{
                    variable_registers[0xF] = 0;

                    println!("Subtracting register {} from {}. Underflowed, setting VF to 0.", y, x);
                } else{
                    variable_registers[0xF] = 1;
                    println!("Subtracting register {} from {}. No underflow, setting VF to 1.", y, x);
                }
            },
            //8XYE
            (8, x, y, 0xE) =>{
                if config.ignore_y_in_8xy_shift_instruction{
                    println!("Ignore Y in 8XY shift instruction");
                } else{
                    println!("Use Y in 8XY shift instruction");
                    variable_registers[x as usize] = variable_registers[y as usize];
                }

                let shifted_out_bit = variable_registers[x as usize] >> 7 & 0b00000001; 
                variable_registers[x as usize] <<= 1;

                variable_registers[0xF] = shifted_out_bit & 1;
            },
            //9XY0
            (9, x, y, 0) =>{
                if variable_registers[x as usize] != variable_registers[y as usize]{
                    println!("Skipping an instruction since {} != {}", variable_registers[x as usize], variable_registers[y as usize]);
                    program_counter += 2;
                } else{
                    println!("Not skipping an instruction since {} == {}", variable_registers[x as usize], variable_registers[y as usize]);
                }
            }
            //ANNN
            (0xA, n0, n1, n2) =>{
                // let value = u16::from_be_bytes([n0, (n1 << 4 | n2)]);
                let value = extract_12_bit_number(n0, n1, n2);

                println!("Set index register I to {}", value);

                set_index_register(&mut ram, value);
            },
            //DXYN
            (0xD, x, y, n0) =>{
                let mut x_coord = variable_registers[x as usize] % 63;
                let mut y_coord = variable_registers[y as usize] % 31;

                variable_registers[0x0f] = 0;

                let index_register_value = get_index_register(&ram);
                for row in 0..n0{
                    x_coord = variable_registers[x as usize] % 63;

                    if y_coord > 31{ break; }

                    let sprite_data = ram[(index_register_value+row as u16) as usize];

                    for bit_shift in (0..=7).rev(){
                        let bit = sprite_data >> bit_shift & 1;

                        if x_coord > 63{ break; }

                        if bit == 1{ 
                            if display.get_pixel_at(x_coord as u32, y_coord as u32){
                                variable_registers[0x0f] = 1;
                            } 
                            
                            display.flip_pixel_on_screen(x_coord as u32, y_coord as u32)
                                .expect(&format!("Could not flip pixels ({}, {}) for DXYN.", x_coord, y_coord));
                        }

                        x_coord += 1;
                    }

                    y_coord += 1;
                }

            },
            //EX9E
            (0xE, x, 9, 0xE) =>{
                let scancodes_pressed = display.get_keypad_press();

                if scancodes_pressed.contains(&x){
                    program_counter += 2;
                }
            },
            //EXA1
            (0xE, x, 0xA, 1) =>{
                let scancodes_pressed = display.get_keypad_press();

                if !scancodes_pressed.contains(&x){
                    program_counter += 2;
                }
            },
            //FX07
            (0xF, x, 0, 7) =>{
                variable_registers[x as usize] = delay_timer;
            },
            //FX15 
            (0xF, x, 1, 5) =>{
                delay_timer = variable_registers[x as usize];
            },
            //FX18  
            (0xF, x, 1, 8) =>{
                sound_timer = variable_registers[x as usize];
            },

            _ =>{
                println!("Unrecognized instruction");
            },
        }

        display.tick();
    }

    
}

pub fn load_fonts(ram: &mut [u8; 4096]){
    let font_data: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    let font_memory_location = 80;
    
    ram[font_memory_location..font_memory_location + font_data.len()]
        .clone_from_slice(&font_data);
}

pub fn set_index_register(ram: &mut [u8; 4096], value: u16){
    let index_register_position = 160;

    ram[index_register_position..index_register_position+2]
        .clone_from_slice(&value.to_be_bytes());
}

pub fn increment_index_register(ram: &mut [u8; 4096], increment: u16){
    let index_register_position = 160;

    let current_value = u16::from_be_bytes([ram[index_register_position], ram[index_register_position+1]]);

    set_index_register(ram, current_value + increment);
}

pub fn get_index_register(ram: &[u8; 4096]) -> u16{
    let index_register_position = 160;
    u16::from_be_bytes([ram[index_register_position], ram[index_register_position+1]])
}

pub fn extract_8_bit_number(n0: u8, n1: u8) -> u8{
    (n0 << 4) + n1
}

pub fn extract_12_bit_number(n0: u8, n1: u8, n2: u8) -> u16{
    u16::from_be_bytes([n0, (n1 << 4 | n2)])
}
