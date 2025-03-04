#![allow(dead_code)]
#![allow(non_snake_case)]
//use crate::chip_8::*;
mod chip_8;

use std::io;
//use std::io::BufReader;
//use std::fs::File;
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

fn main() -> Result<(), io::Error>{
    // The data is stored as big endian.
        
    // Total memory for the emulator.
    
    //let mut memory: Vec<i16> = Vec::with_capacity(4096);
    
    // Total amount of registers: the last one cannot be used in a program.
    
    //let mut registers: Vec<u8> = Vec::with_capacity(16);

    // Special register Used for storing memory addresses.
    
    //let mut i: i16 = 0;
    
    // The program counter.
    // The program counter (PC) should be 16-bit, and is used to store the currently executing address.
    
    //let mut pc: i16 = 0;

    // The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    
    //let mut sp: i8 = 0;

    //  The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine.
    // Chip-8 allows for up to 16 levels of nested subroutines.

    //let mut stack: Vec<i16> = Vec::with_capacity(16);


    // run_chip_8(memory, registers, opcodes, pc);


    /*
    while (¡stop_emulation)
    {
    executeCPU(cycles_to_execute);
    generateInterrupts();
    emulateGraphics();
    emulateSound();
    emulateOtherSoftware();
    timeSincronization();
    }
    Figura 2. Basic Emulator Algorithmu
    */
    println!("Please enter the name of the chip-8 rom:");
    //let filename = "bin/1-chip8-logo.ch8";
    let filename = "bin/4-flags.ch8"; //String::new();
   // let filename = "Cave.ch8";
    //io::stdin().read_line(&mut filename)?;
    //filename = filename.trim_end().to_string();
    //println!("{:?}", filename);
    let mut chip_8 = chip_8::Chip8::new(filename, 640, 320);
    chip_8.run_chip_8();
    //chip_8.run_chip_8(&mut memory, &mut registers, opcodes, program_counter, stack_pointer, &mut stack, address_register);
    //println!("{:?}", chip_8);

    Ok(())
}
