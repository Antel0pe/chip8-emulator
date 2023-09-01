use sdl2::{pixels::Color, event::Event, keyboard::Keycode, video::Window, render::Canvas, Sdl, rect::Rect};
use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 64;
pub const SCREEN_HEIGHT: u32 = 32;
pub const SCALE_FACTOR: u32 = 10;

pub struct Display{
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub pixels: Vec<bool>,
}

impl Display{
    pub fn new() -> Display{
        let sdl_context = sdl2::init().unwrap();
        let video_system = sdl_context.video().unwrap();

        let window = video_system.window("Chip-8 Emulator", SCREEN_WIDTH*SCALE_FACTOR, SCREEN_HEIGHT*SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display { 
            sdl_context,
            canvas,
            pixels: vec![false; (SCREEN_WIDTH*SCREEN_HEIGHT) as usize],
        }
    }

    pub fn clear_screen(&mut self){
        self.canvas.clear();
        self.canvas.present();
    }

    fn get_pixels_xy_idx(&self, x: u32, y: u32) -> usize{
        (y * SCREEN_WIDTH + x) as usize
    }

    pub fn get_pixel_at(&self, x: u32, y: u32) -> bool{
        println!("({}, {})", x, y);
        self.pixels[self.get_pixels_xy_idx(x, y)]
    }

    fn set_pixel_at(&mut self, x: u32, y: u32, value: bool){
        let idx = self.get_pixels_xy_idx(x, y);
        self.pixels[idx] = value;
    }

    fn translate_point_to_rect(&self, x: u32, y: u32) -> Rect{
        Rect::new(
            (x * SCALE_FACTOR) as i32,
            (y * SCALE_FACTOR) as i32,
            SCALE_FACTOR,
            SCALE_FACTOR
        )
    }

    pub fn flip_pixel_on_screen(&mut self, x: u32, y: u32) -> Result<(), String>{
        let scaled_rect = self.translate_point_to_rect(x, y);
        println!("Given coords ({}, {})", x, y);
        println!("Center of drawn rect {:?}", scaled_rect.center());
        let color: Color;

        if self.get_pixel_at(x, y){
            color = Color::BLACK;
            self.set_pixel_at(x, y, false);
        } else{
            color = Color::GREEN;
            self.set_pixel_at(x, y, true);
        }

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(scaled_rect)?;
        self.canvas.present();

        Ok(())
    }

}

