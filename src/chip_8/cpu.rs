//use std::io::BufReader;
// use std::fs::File;
//use rand::prelude::*;

/*
extern crate display;
use display;
use Sound;
use Opcode;

 */

//type Opcode = Vec<u16>;
//type Opcodes = Vec<Opcode>;

//pub mod Cpu;
    use std::{io, fs};

use super::display::Display;


struct Opcode([u16; 4]);
const SPRITE_DATA: [i8; 8 * 16] = [0; 8 * 16];
    pub struct Cpu
    {
        memory: [i16; 4096],
        registers: [u8; 16],
        stack: Vec<i16>, //[i16; 16],
        pub program_counter: i16,
        address_register: i16,
        stack_pointer: i8,
        //delay_timer: i8,
        //sound_timer: i8,
        pub opcodes: Vec<Vec<u16>>,
    }

    impl Cpu
    {
        pub fn new(filename: &str) -> Cpu
        {
            Cpu
            {
                memory: [0; 4096],
                registers: [0; 16],
                stack: [0; 16].to_vec(),
                program_counter: 0,
                address_register: 0,
                stack_pointer: 0,
                opcodes: Cpu::load_instructions(filename).unwrap(),
            }
        }
        fn set_sprite_data_to_memory(&mut self)
        {
            todo!()
        }

        fn retrive_opcode_data(code: u16) -> Vec<u16>
        {
            let mut opcode: Vec<u16> = Vec::with_capacity(4);
            opcode.push(code  >> 12);
            opcode.push((code & 0x0f00)>> 8);
            opcode.push((code & 0x00f0)>> 4);
            opcode.push(code & 0x000f);
            return opcode;
        }


       //TODO: Rework this function using two pointer technique.
        fn load_instructions(filename: &str) -> io::Result<Vec<Vec<u16>>>
        {
            let contents = fs::read(filename)?;

            let mut codes: Vec<u16> = Vec::new();
            for idx in 1..contents.len()
            {
                codes.push(((contents[idx-1] as u16) << 8) | (contents[idx]) as u16);
            }

            let opcodes: Vec<Vec<u16>> = codes.iter().map(|x|
            {
                Cpu::retrive_opcode_data(*x)
            }).collect();

            Ok(opcodes)
        }


        fn key_press_check(&mut self, index: u16, display: &Display, if_pressed: bool) -> ()
        {
            if if_pressed
            {
                if display.key_pressed[index as usize] != -1
                {
                    self.program_counter += 2;
                }
            }
            else
            {
                if display.key_pressed[index as usize] == -1
                {
                    self.program_counter +=2;
                }
            }
        }

        fn key_press_wait(&mut self, index: u16, display: &mut Display)
        {
            display.store_key_value(index.into(), self.registers);
        }

        fn register_load(&mut self, index: &i16)
        {
            for i in 0..=*index
            {
                self.registers[i as usize] = self.memory[(self.address_register + i) as usize] as u8;
            }
        }

        fn register_dump(&mut self, index: &i16)
        {
            for i in 0..=*index
            {
                self.memory[(self.address_register + i) as usize] = self.registers[i as usize] as i16;
            }

        }

        pub fn execute_opcode(&mut self, display: &mut Display, opcode: Vec<u16>)
       {
           //let opcode = Cpu::retrive_opcode_data(opcode);
           match opcode[0] {
               0 => match opcode[2]
               {
                   0 => todo!(), // clear_screen(); Maybe a reference to the window could be used.
                   0xe => 
                       {
                           self.program_counter = *self.stack.last().unwrap();
                           self.stack_pointer -= 1;
                           self.stack.pop();
                       },
                   _ => (),
       
               },
               0x1 => self.program_counter =  (opcode[1] << 8 | opcode[2] << 4 | opcode[3]) as i16,
               0x2 => 
               {
                   self.stack_pointer += 1;
                   self.stack.push(self.program_counter);
                   self.program_counter =  (opcode[1] << 8 | opcode[2] << 4 | opcode[3]) as i16;
               },
               0x3 => 
               {
                   if self.registers[opcode[1] as usize] == (opcode[2] << 4 | opcode[3]) as u8
                   {
                       self.program_counter += 2;
                   }
               },
               0x4 =>
               {
                   if self.registers[opcode[1] as usize] != (opcode[2] << 4 | opcode[3]) as u8
                   {
                       self.program_counter += 2;
                   }
               },
               0x5 => 
               {
                   if self.registers[opcode[1] as usize] == self.registers[opcode[2] as usize]
                   {
                       self.program_counter += 2;
                   }
               },
               0x6 => 
               {
                   self.registers[opcode[1] as usize] = (opcode[2] << 4 | opcode[3]) as u8;
               },
               0x7 => 
               {
                  self.registers[opcode[1] as usize] = 
                       self.registers[opcode[1] as usize] + (opcode[1] << 4 | opcode[2]) as u8;
               },
               0x8 => match opcode[3] 
               {
                   0 => 
                   {
                       self.registers[opcode[1] as usize] = self.registers[opcode[2] as usize];
                   },
                   1 => 
                   {
                       self.registers[opcode[1] as usize] = self.registers[opcode[1] as usize] | self.registers[opcode[2] as usize];
                   },
                   2 => 
                   {
                       self.registers[opcode[1] as usize] = self.registers[opcode[1] as usize] & self.registers[opcode[2] as usize];
                   }, 
                   3 => 
                   {
                       self.registers[opcode[1] as usize] = self.registers[opcode[1] as usize] ^ self.registers[opcode[2] as usize];
                   },
                   4 => 
                   {
                       if (self.registers[opcode[1] as usize] + self.registers[opcode[2] as usize]) as i16 > 255
                       {
                           self.registers[0xf] = 1;
                       }
                       self.registers[opcode[1] as usize] = (self.registers[opcode[1] as usize] + self.registers[opcode[2] as usize]) % 255;
                   },
                   5 => 
                   {
                       if self.registers[opcode[1] as usize] > self.registers[opcode[2] as usize]
                       {
                           self.registers[0xf] = 1;
                       }
                       else {
                           self.registers[0xf] = 0;
                       }
                       self.registers[opcode[1] as usize] = self.registers[opcode[1] as usize] - self.registers[opcode[2] as usize];
                   },
                   6 => 
                   {
                       self.registers[opcode[1] as usize] = (self.registers[opcode[2] as usize] << 4) | (self.registers[opcode[3] as usize]);
                   },
                   7 => 
                   {
                       if self.registers[opcode[2] as usize] > self.registers[opcode[1] as usize]
                       {
                           self.registers[0xf] = 1;
                       }
                       else {
                           self.registers[0xf] = 0;
                       }
                       self.registers[opcode[1] as usize] = self.registers[opcode[2] as usize] - self.registers[opcode[1] as usize];
       
                   },
                   0xe => 
                   {
                       if (self.registers[opcode[1] as usize] >> 7) & 1 == 1
                       {
                           self.registers[0xf] = 1;
                           self.registers[opcode[1] as usize] *= 2;
       
                       }
                       else {
                           self.registers[0xf] = 0;
                       }
                   },
                   _ => todo!(),
               },
               0x9 => 
               {
                   if self.registers[opcode[1] as usize] != self.registers[opcode[2] as usize]
                   {
                       self.program_counter += 2;
                   }
               },
               0xa => 
               {
                   self.address_register = ((opcode[1] << 8) | (opcode[2]) << 4 | (opcode[3])) as i16;
               },
               0xb => // Jump => set the program counter to another addr.
               {
                   self.program_counter = ((opcode[1] << 8) | (opcode[2]  << 4) | (opcode[3]) + (self.registers[0x0] as u16)) as i16;
               },
               0xc => 
               {
                   self.registers[opcode[1] as usize] = rand::random::<u8>() & ((opcode[2] << 4) | (opcode[3])) as u8;
               },
               0xd => 
               {
                   display.draw_on_screen(self.registers[opcode[1] as usize], self.registers[opcode[2] as usize], opcode[3]);
                   
               },
               0xe => // both opcodes need keyboard input -> for later. 
                    match opcode[2]
                    {
                        9 => self.key_press_check(opcode[1], display, true),
                        0xa => self.key_press_check(opcode[1], display, false),
                        _ => (),
                    }
               0xf => 
                   match opcode[2]
                   {
                       0 => self.key_press_wait(opcode[1], display),
                       1 => match opcode[3] {
                           5 => todo!(),
                           8 => todo!(),
                           0xe => self.address_register += (self.registers[opcode[1] as usize]) as i16,
                           _ => (),
                       }
                       2 => todo!(), // Set address_register to the sprite addr for char in vx
                       3 => // bcd of vx in i,  i+1, i+2
                       {
                           self.memory[self.address_register       as usize] = (self.registers[opcode[1] as usize] / 100) as i16;
                           self.memory[(self.address_register + 1) as usize] = (self.registers[opcode[1] as usize] /  10) as i16;
                           self.memory[(self.address_register + 2) as usize] = (self.registers[opcode[1] as usize] %  10) as i16;
                       }, 
                       5 => self.register_dump(&(opcode[1] as i16)),
                       6 => self.register_load( &(opcode[1] as i16)),
                       _ => (),
                   },
               _ => todo!(),
           }
       }
    }
//}
