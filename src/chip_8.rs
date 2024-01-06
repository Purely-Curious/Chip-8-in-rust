mod cpu;
mod display;

    use crate::chip_8::cpu::*;
    use crate::chip_8::display::*;

// #[derive(Debug)]
    pub struct Chip8
    {
        cpu: Cpu,
        display: Display,
    }

    impl Chip8
    {
        pub fn new(filename: &str, window_width: u32, window_height: u32) -> Self
        {
            Self
            {
                cpu: Cpu::new(filename),
                display: Display::new("test".to_string(), window_width, window_height),
            }
        }
        pub fn run_chip_8(&mut self) -> ()
        {
            // Exclusively for the memory;
            let _start_index: i32 = 0x200;
            for _ in 0..self.cpu.opcodes.len() {
                if self.cpu.program_counter > (self.cpu.opcodes.len() as i16)
                {
                    return;
                }
                //let opcode = self.cpu.opcodes[self.cpu.program_counter as usize]
                //self.cpu.execute_opcode(self.cpu.opcodes[self.cpu.program_counter as usize]);
            }
        }

    }
    
