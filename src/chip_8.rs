mod sound;
mod cpu;
mod display;

    type Opcode = Vec<u16>;
    use crate::chip_8::cpu::*;
    use crate::chip_8::display::*;
    use crate::chip_8::sound::*;

#[derive(Debug)]
    pub struct Chip8<'a>
    {
        cpu: Cpu,
        display: Display<'a>,
        sound: Sound,
    }

    impl Chip8<'_>
    {
        pub fn new(filename: &str) -> Self
        {
            Self
            {
                cpu: Cpu::new(filename),
                display: Display::new(),
                sound: Sound::new(),
            }
        }

        pub fn run_chip_8(&self, memory: &mut Vec<i16>, registers: &mut Vec<u8>,
            opcodes: Opcode,  program_counter:&mut  i16, stack_pointer: &mut i8,
          stack: &mut Vec<i16>, address_register: &mut i16) -> ()
        {
            // Exclusively for the memory;
            let _start_index: i32 = 0x200;
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
    