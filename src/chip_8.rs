mod cpu;
mod ppu;
    
use core::time;
use std::thread::sleep;
use std::{io, fs};

use crate::chip_8::cpu::*;
use crate::chip_8::ppu::*;

// #[derive(Debug)]
pub struct Chip8
{
    cpu: Cpu,
    ppu: Ppu,
    rom_name: String,
    //audio_system: Audio,
    memory: [u8; 4096],
    framebuffer: [[i8; 64]; 32],
    inputbuffer: [i8; 16],
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
        Self
        {
            cpu: Cpu::new(),
            ppu: Ppu::new(filename.to_string(), window_width, window_height),
            rom_name: filename.to_string(),
            memory: [0; 4096],
            framebuffer: [[0; 64]; 32],
            inputbuffer: [0; 16],
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
    fn swap_entries(arr: &mut Vec<u8>)
    {
        for i in (0..arr.len()).step_by(2)
        {
            let temp = arr[i];
            arr[i] = arr[i+1];
            arr[i+1] = temp;
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
                println!("{:#x}", contents[i]);
                self.memory[0x200 + i] = contents[i];
            }
            Ok(())
        }

        /*

        The way this will run is that the cpu will execute the instructions and then the ppu will display the updated graphics.
    */
    pub fn run_chip_8(&mut self) -> ()
    {
        // Loads necessary data for the system.
        self.load_sprite_data();

        self.load_rom().expect("The rom should be a valid chip-8 rom.");
        let mut sound_timer: u8 = 60;
        let mut delay_timer: u8 = 60;
        
        // Exclusively for the memory
        for _ in 0..10000//while self.running
        {
            if delay_timer > 0
            {
                delay_timer -= 1;
            }
            if sound_timer == 0
            {
                sound_timer = 60;
            }
            else {
                sound_timer-=1;
            }

            self.cpu.execute(&mut self.memory, &mut self.framebuffer, &mut self.inputbuffer, &self.ppu.sdl_context, &mut delay_timer, &mut sound_timer);
            self.ppu.update_graphics(&self.framebuffer);
            // have a function to get the keyboard input.   
           // println!("{:?}", self.inputbuffer);
           sleep(time::Duration::from_millis(1));
            //self.audio_system.play_audio(&mut self.cpu.sound_timer);
        }
    }

}
    
