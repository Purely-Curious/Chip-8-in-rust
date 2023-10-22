use std::io::BufReader;
use std::fs::File;
use rand::prelude::*;
/// ---- Opcode legend ----
///    NNN: address
///    NN: 8-bit constant
///    N: 4-bit constant
///    X and Y: 4-bit register identifier
///    PC : Program Counter
///    I : 16bit register (For memory address) (Similar to void pointer);
///    VN: One of the 16 available variables. N may be 0 to F (hexadecimal);
type Opcode = i16;
// . However, the CHIP-8 interpreter itself occupies the first 512 bytes of the memory space on these machines. 
//  For this reason, most programs written for the original system begin at memory location 512 (0x200) and do
//  not access any of the memory below the location 512 (0x200). The uppermost 256 bytes (0xF00-0xFFF) are 
//  reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) were reserved for the call stack,
//  internal use, and other variables. 

/* Display coords. range -> x: (0, 63), y: (0, 31)

    (0,0)	(63,0)
    (0,31)	(63,31)
 */
fn retrive_opcode_data(opcode: Opcode) -> Vec<Opcode>
{
    let mut processed_opcode: Vec<Opcode> = Vec::with_capacity(4);
    processed_opcode.push((opcode  >> 12));
    processed_opcode.push((opcode & 0x0f00)>> 8);
    processed_opcode.push((opcode & 0x00f0)>> 4);
    processed_opcode.push((opcode & 0x000f));
    return processed_opcode;
}

fn execute_opcode(opcode: Opcode, memory: &mut Vec<i16>, registers: &mut Vec<u8>, address_register: &mut i16,
     program_counter: &mut i16, stack_pointer: &mut i8, stack: &mut Vec<i16>)
{
    let processed_opcode = retrive_opcode_data(opcode);
    match processed_opcode[0] {
        0 => match processed_opcode[2]
        {
            0 => todo!(),
            0xe => 
                {
                    *program_counter = *stack.last().unwrap();
                    *stack_pointer -= 1;
                },
            _ => (),

        },
        0x1 => *program_counter =  (processed_opcode[1] << 8 | processed_opcode[2] << 4 | processed_opcode[3]),
        0x2 => 
        {
            *stack_pointer += 1;
            stack.push(*program_counter);
            *program_counter =  (processed_opcode[1] << 8 | processed_opcode[2] << 4 | processed_opcode[3]);
        },
        0x3 => 
        {
            if i16::from(registers[processed_opcode[1] as usize]) == processed_opcode[2] << 4 | processed_opcode[3]
            {
                *program_counter += 2;
            }
        },
        0x4 =>
        {
            if i16::from(registers[processed_opcode[1] as usize]) != processed_opcode[2] << 4 | processed_opcode[3]
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
            *program_counter = (processed_opcode[1] << 8) | (processed_opcode[2]  << 4) | (processed_opcode[3]) + (registers[0x0] as i16);
        },
        0xc => 
        {
            registers[processed_opcode[1] as usize] = rand::random<i8>() && ((processed_opcode[2] << 4) | (processed_opcode[3]));
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
                    
                }
                2 => todo!(), // Set address_register to the sprite addr for char in vx
                3 => todo!(), // bcd of vx in i,  i+1, i+2
                5 => register_dump(memory, registers, address_register),
                6 => register_load(memory, registers, address_register),
            },
        _ => todo!(),
    }
}


// Some graphics library will be required for visuals. 
// TODO: create the graphical system after the implementation of the emulator is complete.

fn run_chip_8(memory: &mut Vec<i16>, registers: &mut Vec<u8>,
              opcodes: Vec<Opcode>,  program_counter:&mut  i16, stack_pointer: &mut i8,
            stack: &mut Vec<i16>, address_register: &mut i16) -> ()
{
    // Exclusively for the memory;
    let start_index: i32 = 0x200;
    for _ in 0..opcodes.len(){
        if *program_counter > (opcodes.len() as i16)
        {
            return;
        }
        execute_opcode(opcodes[*program_counter as usize], memory, registers, address_register, program_counter, stack_pointer, stack);
    }
    todo!();
}
fn main() {
    // The data is stored as big endian.
        
    // Total memory for the emulator.
    let mut memory: Vec<i16> = Vec::with_capacity(4096);
    // Total amount of registers: the last one cannot be used in a program.
    let mut registers: Vec<u8> = Vec::with_capacity(16);

    // Special register Used for storing memory addresses.
    let mut i: i16 = 0;
    
    // The program counter.
    // The program counter (PC) should be 16-bit, and is used to store the currently executing address.
    let mut pc: i16 = 0;

    // The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    let mut sp: i8 = 0;

    //  The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine.
    // Chip-8 allows for up to 16 levels of nested subroutines.

    let mut stack: Vec<i16> = Vec::with_capacity(16);


    // run_chip_8(memory, registers, opcodes, pc);

    println!("Hello, world!");
}
