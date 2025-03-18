mod cpu;
mod ppu;
mod audio;

//use std::thread::sleep;
//use std::time::Duration;
use std::{fs, io, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::chip_8::cpu::*;
use crate::chip_8::ppu::*;
use crate::chip_8::audio::*;
// #[derive(Debug)]
pub struct Chip8
{
    cpu: Cpu,
    ppu: Ppu,
    rom_name: String,
    audio_system: Apu,
    memory: [u8; 4096],
    framebuffer: [[i8; 64]; 32],
    inputbuffer: [i8; 16],
    sdl_context: Sdl,
    running: bool,
}


const SPRITE_DATA: [u8; 16 * 5] = [
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



impl Chip8
{
    pub fn new(filename: &str, window_width: u32, window_height: u32) -> Self
    {
        let sdl_context = sdl2::init().unwrap(); 
        Self
        {
            cpu: Cpu::new(),
            ppu: Ppu::new(filename.to_string(), window_width, window_height, &sdl_context),
            rom_name: filename.to_string(),
            audio_system: Apu::new(&sdl_context),
            memory: [0; 4096],
            framebuffer: [[0; 64]; 32],
            inputbuffer: [0; 16],
            sdl_context: sdl_context,
            running: true,
        }
    }
    fn load_sprite_data(&mut self)
    {
        let address_space_start = 0x50;
        for i in 0..SPRITE_DATA.len()
        {
            self.memory[address_space_start + i] = SPRITE_DATA[i];
        }
    }
    // 1 - 2
    // 2 - 1
    fn load_rom(&mut self) -> io::Result<()>
        {
            let contents = fs::read(self.rom_name.clone())?;
            // required to correct the byte reordering.
            //Chip8::swap_entries(&mut contents);


            for i in 0..contents.len()
            {
                self.memory[0x200 + i] = contents[i];
            }
            Ok(())
        }

        fn process_input(&mut self, key_pressed: &mut bool) {
            // somehow make this cleaner?
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { self.running = false; },
                    Event::KeyDown {keycode: Some(Keycode::Num1), ..} => { self.inputbuffer[0x1] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::Num2), ..} => { self.inputbuffer[0x2] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::Num3), ..} => { self.inputbuffer[0x3] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::Num4), ..} => { self.inputbuffer[0xc] = 1; *key_pressed = true; },  //return; },; },                    
                    Event::KeyDown {keycode: Some(Keycode::Q), ..} => { self.inputbuffer[0x4] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::W), ..} => { self.inputbuffer[0x5] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::E), ..} => { self.inputbuffer[0x6] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::R), ..} => { self.inputbuffer[0xd] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::A), ..} => { self.inputbuffer[0x7] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::S), ..} => { self.inputbuffer[0x8] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::D), ..} => { self.inputbuffer[0x9] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::F), ..} => { self.inputbuffer[0xe] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::Z), ..} => { self.inputbuffer[0xa] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::X), ..} => { self.inputbuffer[0x0] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::C), ..} => { self.inputbuffer[0xb] = 1; *key_pressed = true; },  //return; },; },
                    Event::KeyDown {keycode: Some(Keycode::V), ..} => { self.inputbuffer[0xf] = 1; *key_pressed = true; },  //return; },; },

                    Event::KeyUp {keycode: Some(Keycode::Num1), ..} => { self.inputbuffer[0x1] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::Num2), ..} => { self.inputbuffer[0x2] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::Num3), ..} => { self.inputbuffer[0x3] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::Num4), ..} => { self.inputbuffer[0xc] = 0; *key_pressed = false; },  //return; },; },                    
                    Event::KeyUp {keycode: Some(Keycode::Q), ..} => { self.inputbuffer[0x4] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::W), ..} => { self.inputbuffer[0x5] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::E), ..} => { self.inputbuffer[0x6] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::R), ..} => { self.inputbuffer[0xd] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::A), ..} => { self.inputbuffer[0x7] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::S), ..} => { self.inputbuffer[0x8] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::D), ..} => { self.inputbuffer[0x9] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::F), ..} => { self.inputbuffer[0xe] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::Z), ..} => { self.inputbuffer[0xa] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::X), ..} => { self.inputbuffer[0x0] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::C), ..} => { self.inputbuffer[0xb] = 0; *key_pressed = false; },  //return; },; },
                    Event::KeyUp {keycode: Some(Keycode::V), ..} => { self.inputbuffer[0xf] = 0; *key_pressed = false; },  //return; },; },
                    _ => (),
                }
            }   
        }
       /* 

        The way this will run is that the cpu will execute the instructions and then the ppu will display the updated graphics.
    */

    fn play_audio()
    {
        //
    }
    pub fn run_chip_8(&mut self) -> ()
    {
        // need to pass in the sdl context from this file to both the ppu and the audio system.
        
        // Loads necessary data for the system.
        self.load_sprite_data();

        self.load_rom().expect("The rom should be a valid chip-8 rom.");
        let mut sound_timer: u8 = 60;
        let mut delay_timer: u8 = 60;
        let mut key_pressed = false;

        let mut current_time = time::Instant::now();
        let cycle_delay = time::Duration::from_millis(16);
        
        while self.running
        {
            let elapsed_time = time::Instant::now();
            // an optimal number of instructions is 400 per frame.
            let number_of_instructions = 400;
            let time_difference = elapsed_time - current_time;

            // get keyboard input each frame, rather than each instruction cycle.
            self.process_input(&mut key_pressed);

            if time_difference > cycle_delay 
            {
                for _ in 0..number_of_instructions/60 {                    
                    //    self.process_input(&mut key_pressed);
                    self.cpu.execute(&mut self.memory, &mut self.framebuffer, &mut self.inputbuffer, &mut delay_timer, &mut sound_timer, &mut key_pressed);
                    //self.audio_system.play_audio(&mut sound_timer);
                    //println!("{:?}", self.inputbuffer);
                }
                self.ppu.update_graphics(&self.framebuffer);
                
                if delay_timer > 0
                {
                    delay_timer -= 1;
                }
                
                if sound_timer > 0
                {
                    self.audio_system.play_audio(&mut sound_timer);

                    sound_timer-=1;
                }
                current_time = elapsed_time;
            }
        }
    }

}
    
