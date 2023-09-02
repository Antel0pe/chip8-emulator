use std::{fs, time::Duration};

use crate::display::Display;

mod display;

fn main() {
    /*
     * 80 - 159 = Font data
     * 160 - 162 = Index Register
     */
    let mut ram: [u8; 4096] = [0; 4096];

    let mut program_counter: u16 = 512; 
    // let mut index_register: u16 = 0;
    let mut stack: u16 = 0; // really should be part of main memory 
    let mut delay_timer: u8 = 0;
    let mut sound_timer: u8 = 0;
    let mut variable_registers: [u8; 16] = [0; 16];

    let file_path = "roms/IBM Logo.ch8";

    let content = fs::read(file_path)
        .expect(&format!("Could not read ROM: {}", file_path));

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

    let nibble_instructions = content.chunks(2)
        .map(|b| (b[0] >> 4, b[0] & 0b00001111, b[1] >> 4, b[1] & 0b00001111))
        .fold(Vec::new(), |mut acc, e| {
            acc.push(e);
            acc
        });
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
            (0, 0, 14, 0) =>{
                display.clear_screen();
                println!("Clear screen!");
            },
            // 1NNN
            (1, n0, n1, n2) =>{
                let address = u16::from_be_bytes([n0, (n1 << 4 | n2)]);
                println!("Jump to {}", address);

                program_counter = address;

                // println!("Remove if not working on IBM Logo");
                // ::std::thread::sleep(Duration::new(1000, 1));
                // break; // TEMP BECAUSE IBM LOGO REPEATS HERE
            },
            //6XNN
            (6, x, n0, n1) =>{
                println!("Set register V{} to {}", x, (n0 << 4) + n1);

                variable_registers[x as usize] = (n0 << 4) + n1;
            },
            //7XNN
            (7, x, n0, n1) =>{
                println!("To register V{} add {}", x, (n0 << 4) + n1);

                variable_registers[x as usize] += (n0 << 4) + n1;
            },
            //ANNN
            (10, n0, n1, n2) =>{
                let value = u16::from_be_bytes([n0, (n1 << 4 | n2)]);

                println!("Set index register I to {}", value);

                set_index_register(&mut ram, value);
            },
            //DXYN
            (13, x, y, n0) =>{
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

