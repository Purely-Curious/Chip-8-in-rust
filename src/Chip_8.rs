mod Cpu;
mod Display;
mod Sound;

pub mod Chip_8
{
    type Opcode = Vec<u16>;
    use crate::Chip_8::Cpu::*;
    use crate::Chip_8::Display::*;
    use crate::Chip_8::Sound::*;

    pub struct Chip_8<'a>
    {
        cpu: Cpu,
        display: Display<'a>,
        sound: Sound,
    }

    impl Chip_8<'_>
    {
        pub fn new(filename: &str) -> Chip_8
        {
            Chip_8
            {
                cpu: Cpu::new(filename),
                display: Display::new(),
                sound: Sound::new(),
            }
        }

        fn run_chip_8(&self, memory: &mut Vec<i16>, registers: &mut Vec<u8>,
            opcodes: Opcode,  program_counter:&mut  i16, stack_pointer: &mut i8,
          stack: &mut Vec<i16>, address_register: &mut i16) -> ()
        {
            // Exclusively for the memory;
            let start_index: i32 = 0x200;
            for _ in 0..opcodes.len(){
                if *program_counter > (opcodes.len() as i16)
                {
                    return;
                }
                
                self.cpu.execute_opcode(opcodes[*program_counter as usize], memory, registers, address_register, program_counter, stack_pointer, stack);
            }
            todo!();
        }

    }
    
}