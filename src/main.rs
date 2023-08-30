use std::{fs, time::Duration};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode};

fn main() {
    /*
     * 80 - 159 = Font data
     * 160 - 162 = Index Register
     */
    let mut ram: [u8; 4096] = [0; 4096];

    let mut program_counter: u16 = 0; 
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

    let instructions = content.chunks(2)
        .map(|b| format!("{:02X?}{:02X?}", b[0], b[1]).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("ROM Instructions {}", instructions);

    let hex_instructions = content.chunks(2)
        .map(|b| [b[0], b[1]])
        .map(|b| u16::from_be_bytes(b))
        .collect::<Vec<u16>>();
    println!("Hex ROM Instructions {:?}", hex_instructions);

    // println!("1st instruction: {}", 0b1000000000000000 & hex_instructions[1]);
    // println!("2nd instruction: {}", 0b0000100000000000 & hex_instructions[1]);
    // println!("3rd instruction: {}", 0b0000000010000000 & hex_instructions[1]);
    // println!("4th instruction: {}", 0b0000000000001000 & hex_instructions[1]);

    let individual_u4_instructions = content.iter()
        .map(|b| [b >> 4, b & 0b00001111])
        .fold(Vec::new(), |mut acc, e| {
            acc.extend(e);
            acc
        });
    println!("'U4' Instructions: {:?}", individual_u4_instructions);




    // load memory in to 512 in decimal
    ram[512..512+content.len()]
        .clone_from_slice(&content);

    // println!("RAM Contents: {:?}", ram);

    // show_screen();

    // load_fonts(&mut ram);



    
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

pub fn show_screen(){
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();

    let window = video_system.window("Chip-8 Emulator", 64*10, 32*10)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
