extern crate sdl2; 

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;



 
 // const WHITE: Color = Color::WHITE;

    pub struct Display
    {
        title: String,
        window_height: u32,
        window_width: u32,
        framebuffer: [i16; 64 * 32],
        canvas: Canvas<Window>,
        event_pump: EventPump,
        pub key_pressed: [i8; 16],
    }

    impl Display 
    {
        pub fn new(title: String, window_width: u32, window_height: u32) -> Display
        {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem.window("Chip-8 emulator", window_width, window_height)
                    .position_centered()
                    .build()
                    .unwrap();

            let canvas = window.into_canvas()
            .build().unwrap();

            let event_pump = sdl_context.event_pump().unwrap();


            Display
            {
                title,
                window_height,
                window_width,
                framebuffer: [0; 64*32],
                canvas,
                event_pump,
                key_pressed: [-1; 16],
            }

        }
        //pub fn draw_on_screen(&mut self, x: u8, y: u8, nibble: u16){}
            //draw(registers[opcode[1] as usize], registers[opcode[2] as usize], opcode[3]);
                   /*Dxyn - DRW Vx, Vy, nibble
                   Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
       
                   The interpreter reads n bytes from memory, starting at the address stored in I.
                   These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
                   Sprites are XORed onto the existing screen.
                   If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                   If the sprite is positioned so part of it is outside the coordinates of the display, 
                   it wraps around to the opposite side of the screen.
                   See instruction 8xy3 for more information on XOR, 
                   and section 2.4, Display, for more information on the Chip-8 screen and sprites. */
    }
