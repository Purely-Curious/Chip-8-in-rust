//use std::io::BufReader;
// use std::fs::File;
use rand::prelude::*;

/*
extern crate display;
use display;
use Sound;
use Opcode;

 */

type Opcode = Vec<u16>;
type Opcodes = Vec<Opcode>;

//pub mod Cpu;
    use std::{io, fs, default};



    pub struct Cpu
    {
        framebuffer: [i16; 64 * 32],
        opcodes: Vec<Vec<u16>>,
    }
    impl Cpu
    {
        pub fn new(filename: &str) -> Cpu
        {
            Cpu
            {
                framebuffer: [0; 64 * 32],
                opcodes: Cpu::load_instructions(filename).unwrap(),
            }
        }

        fn retrive_opcode_data(opcode: u16) -> Vec<u16>
        {
            let mut processed_opcode: Vec<u16> = Vec::with_capacity(4);
            processed_opcode.push((opcode  >> 12));
            processed_opcode.push((opcode & 0x0f00)>> 8);
            processed_opcode.push((opcode & 0x00f0)>> 4);
            processed_opcode.push((opcode & 0x000f));
            return processed_opcode;
        }


       
        fn load_instructions(filename: &str) -> io::Result<Vec<Vec<u16>>>
        {
            let contents = fs::read(filename)?;
            let copy_of_contents = contents.clone();
            //let c_contents = copy_of_contents; //.iter().skip(1);
            let results = contents.iter()
            .zip(copy_of_contents
                .iter()
                .skip(1));

            let codes: Vec<u16> = results.map(|x| {
                (*(x.0) as u16) << 8 | (*(x.1) as u16 )
            }).collect();

            let opcodes: Vec<Vec<u16>> = codes.iter().map(|x|
            {
                Cpu::retrive_opcode_data(*x)
            }).collect();
            Ok(opcodes)
        }


        fn register_load(memory:  &mut Vec<i16>, registers: &mut Vec<u8>, address_register: &mut i16, index: &i16)
        {
            for i in 0..=*index
            {
                registers[i as usize] = memory[(*address_register + i) as usize] as u8;
            }
        }

        fn register_dump(memory:  &mut Vec<i16>, registers: &mut Vec<u8>, address_register: &mut i16, index: &i16)
        {
            for i in 0..=*index
            {
                memory[(*address_register + i) as usize] = registers[i as usize] as i16;
            }

        }

        pub fn execute_opcode(&self, opcode: u16, memory: &mut Vec<i16>, registers: &mut Vec<u8>, address_register: &mut i16,
            program_counter: &mut i16, stack_pointer: &mut i8, stack: &mut Vec<i16>)
       {
           let processed_opcode = Cpu::retrive_opcode_data(opcode);
           match processed_opcode[0] {
               0 => match processed_opcode[2]
               {
                   0 => todo!(), // clear_screen(); Maybe a reference to the window could be used.
                   0xe => 
                       {
                           *program_counter = *stack.last().unwrap();
                           *stack_pointer -= 1;
                           stack.pop();
                       },
                   _ => (),
       
               },
               0x1 => *program_counter =  (processed_opcode[1] << 8 | processed_opcode[2] << 4 | processed_opcode[3]) as i16,
               0x2 => 
               {
                   *stack_pointer += 1;
                   stack.push(*program_counter);
                   *program_counter =  (processed_opcode[1] << 8 | processed_opcode[2] << 4 | processed_opcode[3]) as i16;
               },
               0x3 => 
               {
                   if registers[processed_opcode[1] as usize] == (processed_opcode[2] << 4 | processed_opcode[3]) as u8
                   {
                       *program_counter += 2;
                   }
               },
               0x4 =>
               {
                   if registers[processed_opcode[1] as usize] != (processed_opcode[2] << 4 | processed_opcode[3]) as u8
                   {
                       *program_counter += 2;
                   }
               },
               0x5 => 
               {
                   if registers[processed_opcode[1] as usize] == registers[processed_opcode[2] as usize]
                   {
                       *program_counter += 2;
                   }
               },
               0x6 => 
               {
                   registers[processed_opcode[1] as usize] = (processed_opcode[2] << 4 | processed_opcode[3]) as u8;
               },
               0x7 => 
               {
                  registers[processed_opcode[1] as usize] = 
                       registers[processed_opcode[1] as usize] + (processed_opcode[1] << 4 | processed_opcode[2]) as u8;
               },
               0x8 => match processed_opcode[3] 
               {
                   0 => 
                   {
                       registers[processed_opcode[1] as usize] = registers[processed_opcode[2] as usize];
                   },
                   1 => 
                   {
                       registers[processed_opcode[1] as usize] = registers[processed_opcode[1] as usize] | registers[processed_opcode[2] as usize];
                   },
                   2 => 
                   {
                       registers[processed_opcode[1] as usize] = registers[processed_opcode[1] as usize] & registers[processed_opcode[2] as usize];
                   }, 
                   3 => 
                   {
                       registers[processed_opcode[1] as usize] = registers[processed_opcode[1] as usize] ^ registers[processed_opcode[2] as usize];
                   },
                   4 => 
                   {
                       if (registers[processed_opcode[1] as usize] + registers[processed_opcode[2] as usize]) as i16 > 255
                       {
                           registers[0xf] = 1;
                       }
                       registers[processed_opcode[1] as usize] = (registers[processed_opcode[1] as usize] + registers[processed_opcode[2] as usize]) % 255;
                   },
                   5 => 
                   {
                       if (registers[processed_opcode[1] as usize] > registers[processed_opcode[2] as usize])
                       {
                           registers[0xf] = 1;
                       }
                       else {
                           registers[0xf] = 0;
                       }
                       registers[processed_opcode[1] as usize] = (registers[processed_opcode[1] as usize] - registers[processed_opcode[2] as usize]);
                   },
                   6 => 
                   {
                       registers[processed_opcode[1] as usize] = (registers[processed_opcode[2] as usize] << 4) | (registers[processed_opcode[3] as usize]);
                   },
                   7 => 
                   {
                       if (registers[processed_opcode[2] as usize] > registers[processed_opcode[1] as usize])
                       {
                           registers[0xf] = 1;
                       }
                       else {
                           registers[0xf] = 0;
                       }
                       registers[processed_opcode[1] as usize] = (registers[processed_opcode[2] as usize] - registers[processed_opcode[1] as usize]);
       
                   },
                   0xe => 
                   {
                       if (registers[processed_opcode[1] as usize]) >> 7 & 1 == 1
                       {
                           registers[0xf] = 1;
                           registers[processed_opcode[1] as usize] *= 2;
       
                       }
                       else {
                           registers[0xf] = 0;
                       }
                   },
                   _ => todo!(),
               },
               0x9 => 
               {
                   if (registers[processed_opcode[1] as usize] != registers[processed_opcode[2] as usize])
                   {
                       *program_counter += 2;
                   }
               },
               0xa => 
               {
                   *address_register = ((processed_opcode[1] << 8) | (processed_opcode[2]) << 4 | (processed_opcode[3])) as i16;
               },
               0xb => // Jump => set the program counter to another addr.
               {
                   *program_counter = ((processed_opcode[1] << 8) | (processed_opcode[2]  << 4) | (processed_opcode[3]) + (registers[0x0] as u16)) as i16;
               },
               0xc => 
               {
                   registers[processed_opcode[1] as usize] = rand::random::<u8>() & ((processed_opcode[2] << 4) | (processed_opcode[3])) as u8;
               },
               0xd => 
               {
                   //draw(registers[processed_opcode[1] as usize], registers[processed_opcode[2] as usize], processed_opcode[3]);
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
               },
               0xe => // both opcodes need keyboard input -> for later. 
               todo!(),
               0xf => 
                   match processed_opcode[2]
                   {
                       1 => match processed_opcode[3] {
                           5 => todo!(),
                           8 => todo!(),
                           0xe => *address_register += (registers[processed_opcode[1] as usize]) as i16,
                           _ => (),
                       }
                       2 => todo!(), // Set address_register to the sprite addr for char in vx
                       3 => // bcd of vx in i,  i+1, i+2
                       {
                           memory[*address_register       as usize] = (registers[processed_opcode[1] as usize] / 100) as i16;
                           memory[(*address_register + 1) as usize] = (registers[processed_opcode[1] as usize] /  10) as i16;
                           memory[(*address_register + 2) as usize] = (registers[processed_opcode[1] as usize] %  10) as i16;
                       }, 
                       5 => Cpu::register_dump(memory, registers, address_register, &(processed_opcode[1] as i16)),
                       6 => Cpu::register_load(memory, registers, address_register, &(processed_opcode[1] as i16)),
                       _ => (),
                   },
               _ => todo!(),
           }
       }
    }
//}