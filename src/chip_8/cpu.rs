
// ar: Address Register
// sp: Stack pointer
// pc: Program counter
// dt: Delay timer
// st: Sound timer

    pub struct Cpu
    {
        regs: [u8; 16],
        stack: [i16; 16], 
        pc: i16,
        ar: i16,
        sp: usize,
        //pause: bool,
        //dt: i8,
        //st: i8,
    }

    impl Cpu
    {
        pub fn new() -> Cpu
        {
            Cpu
            {
                regs: [0; 16],
                stack: [0; 16],
                pc: 0x200,
                ar: 0,
                sp: 0,
            }
        }
        

        fn retrive_opc_data(&self, mem: &[u8; 4096]) -> Vec<u16>
        {
            let mut op: Vec<u16> = Vec::with_capacity(4);
            op.push(((mem[self.pc as usize   ] & 0xf0) >> 4).into());
            op.push((mem[self.pc as usize   ] & 0x0f).into());
            op.push(((mem[self.pc as usize + 1] & 0xf0) >> 4).into());
            op.push((mem[self.pc as usize + 1] & 0x0f).into());

            op
        }

        // so for some reason the key press function is ran but not the key log function.
        fn key_press_check(&mut self, idx: u8, if_pressed: bool, ib: &[i8; 16]) -> ()
        {
            if if_pressed
            {
                if ib[idx as usize] == 1
                {
                    self.pc += 2;
                }
            }
            else
            {
                if ib[idx as usize] == 0
                {
                    self.pc += 2;
                }
            }
        }

        fn key_press_wait(&mut self, idx: usize, ib: &mut [i8; 16], key_pressed: &mut bool)
        {   // need to put the key_press logic in here.
            for i in 0..16 {
                // if the key is down but it was formerly pressed then set the register with the index to the value of i.
                if ib[i as usize] == 0 && *key_pressed {
                    self.regs[idx] = i;
                    //*key_pressed = true;
                    return;
                }
            }
            self.pc -= 2;
        }

        fn register_load(&mut self, mem: &[u8; 4096], idx: &i16)
        {
            for i in 0..=*idx
            {
                self.regs[i as usize] = mem[(self.ar + i) as usize];
            }
            self.ar += *idx + 1;
        }

        // hmm..
        fn register_dump(&mut self, mem: &mut [u8; 4096], idx: &i16)
        {
            for i in 0..=*idx
            {
                mem[(self.ar + i) as usize] = self.regs[i as usize];
            }
            self.ar += *idx + 1;

        }
        fn clear_screen(&self, fb: &mut [[i8; 64]; 32])
        {
            *fb = [[0; 64]; 32];
        }

        // works for aligned pixels but not for unaligned for some reason....
        fn draw_on_screen(&mut self, mem: &[u8; 4096], fb: &mut [[i8; 64]; 32], x: u8, y: u8, nibble: u16)
        {
            // vx and vy are already passed into this function...
            let mut row;
            let mut col = y % 32;
            self.regs[0xf] = 0;

            for i in self.ar..(self.ar + nibble as i16)
            {
                let sprite_byte = mem[i as usize];
                if col > 31 {
                    break;
                }
                
                row = x % 64;
                for j in 0..8
                {
                    if row > 63 {
                        break;
                    }
                    let pixel = ((sprite_byte >> (7 - j)) & 1) as i8;
                    fb[col as usize][row as usize] ^= pixel;
                    // if the pixel in the framebuffer is set to 0 (i.e. there was a collision)
                    // set VF to 1;
                    if fb[col as usize][row as usize] == 0
                    {
                        self.regs[0xf] = 1;
                    }
                    
                    row = (row + 1) % 64;
                }
                col = col + 1;
            }
        }


       fn debug_display(&self, op: Vec<u8>)
       {
           println!("Current value of program counter: {} \nCurrent opcode: {:?}", self.pc, op);
           println!("Value of registers: {:?}", self.regs);
           println!();
       }

       pub fn execute(&mut self, mem: &mut [u8; 4096], fb: &mut [[i8; 64]; 32], ib: &mut [i8; 16], dt: &mut u8, st: &mut u8, key_pressed: &mut bool)
       {
           let op = self.retrive_opc_data(&mem);
            //self.debug_display(op.clone());

           let x = op[1];
           let y = op[2];
           let vx = self.regs[x as usize];
           let vy = self.regs[y as usize];
           let n = op[3];
           let kk = y << 4 | n; 
           let nnn = x << 8 | kk;
           self.pc +=2;

           
           
           match op[0] {
               0 => match op[3]
               {
                // 00e0
                   0 => {
                    self.clear_screen(fb);
                    }, // clear_screen(); Maybe a reference to the window could be used.
                   //0x00ee
                   0xe => 
                       {
                           self.pc = self.stack[self.sp];
                           self.stack[self.sp] = 0;
                           self.sp -= 1;
                       },
                   _ => return,
       
               },
               //1xxx
               0x1 => self.pc =  nnn as i16,
               //2xxx
               0x2 => 
               {
                   self.sp += 1;
                   self.stack[self.sp] = self.pc;
                   //self.sp += 1;
                   self.pc =  nnn as i16;
               },
               //3xxx
               0x3 => 
               {
                   if vx == kk as u8
                   {
                       self.pc += 2;
                   }
               },
               //4xxx
               0x4 =>
               {
                   if vx != kk as u8
                   {
                       self.pc += 2;
                   }
               },
               //5xxx
               0x5 => 
               {
                   if vx == vy
                   {
                       self.pc += 2;
                   }
               },
               //6xxx
               0x6 => 
               {
                   self.regs[op[1] as usize] = (kk & 0xff) as u8;
               },
               //7xxx
               0x7 => 
               {
                  self.regs[op[1] as usize] = ((vx as u16 + kk) & 0xff) as u8;
               },
               //8xxx
               0x8 => match op[3] 
               {
                //8xx0
                   0 => 
                   {
                       self.regs[op[1] as usize] = vy;
                   },
                   //8xx1
                   1 => 
                   {
                       self.regs[0xf] = 0;
                       self.regs[op[1] as usize] = vx | vy;
                   },
                   //8xx2
                   2 => 
                   {
                    self.regs[0xf] = 0;
                       self.regs[op[1] as usize] = vx & vy;
                   }, 
                   //8xx3
                   3 => 
                   {
                    self.regs[0xf] = 0;
                       self.regs[op[1] as usize] = vx ^ vy;
                   },
                   //8xx4
                   4 => 
                   {
                       self.regs[op[1] as usize] = ((vx as i16 + vy as i16) & 0xff) as u8;  
                       if vx as i16 + vy as i16 > 255
                       {
                           self.regs[0xf] = 1;
                       }
                       else 
                       {
                           self.regs[0xf] = 0;    
                       }
                   },
                   5 => 
                   {
                       self.regs[op[1] as usize] = ((vx as i16 - vy as i16) & 0xff) as u8;
                       
                       if vx >= vy
                       {
                           self.regs[0xf] = 1;
                       }
                       else {
                           self.regs[0xf] = 0;
                       }
                   },
                   //8xx6
                   6 => 
                   {
                       self.regs[x as usize] = vy >> 1;

                        if (vy & 1) == 1 
                        {
                            self.regs[0xf] = 1;
                        }
                        else 
                        {
                            self.regs[0xf] = 0;
                        }
                   },
                 //8xx7
                   7 => 
                   {
                       self.regs[op[1] as usize] = ((vy as i16 - vx as i16) & 0xff) as u8;

                       if vy >= vx
                       {
                           self.regs[0xf] = 1;
                       }
                       else 
                       {
                           self.regs[0xf] = 0;
                       }       
                   },
                   //8xxe
                   0xe => 
                   {
                       self.regs[x as usize] = ((vy as i16) << 1 & 0xff) as u8;

                       if ((vy >> 7) & 1) == 1
                        {
                           self.regs[0xf] = 1;
                       }
                       else 
                       {
                           self.regs[0xf] = 0;
                       }

                   },
                   _ => (),
               },
               //9xxx
               0x9 => 
               {
                   if vx != vy
                   {
                       self.pc += 2;
                   }
               },
               //axxx
               0xa => 
               {
                   self.ar = nnn as i16;
               },
               0xb => // Jump => set the program counter to another addr. vx = nnn + V0
               {
                   self.pc = (nnn + (self.regs[0x0] as u16)) as i16;
               },
               0xc => 
               {
                   self.regs[op[1] as usize] = rand::random::<u8>() & kk as u8;
               },
               //dxxx
               0xd => 
               {
                   self.draw_on_screen(&mem, fb, vx, vy, n);
                   
               },
               //exxx
               0xe => // both ops need keyboard input -> for later. 
                    match op[2]
                    {
                        9 => self.key_press_check(vx, true, &ib),
                        0xa => self.key_press_check(vx, false, &ib),
                        _ => (),
                    }
                //fxxx
               0xf => 
                   match op[2]
                   {
                    //fx0x
                       0 => match op[3]
                       {
                        //fx07
                            7 => 
                            {
                                self.regs[op[1] as usize] = *dt;
                            },
                            //fx0a
                            0xa => 
                            {
                                // need a flag to see if the key is pressed or not.
                                self.key_press_wait(x as usize, ib, key_pressed);
                            },
                            _ => (),
                       },
                       //fx1x
                       1 => match op[3] {
                           5 => *dt = vx, // Sets the delay timer to vx
                           8 => *st = vx, // Sets the sound timer to vx
                           0xe => self.ar += vx as i16, // I + vx
                           _ => (),
                       }
                       //fx2x
                       2 => 
                       {
                        // maybe i dont know
                        self.ar = 0x50 + (vx as i16 * 8);//((vx as i16) / 5) % 16;
                       }, // Set ar to the sprite addr for char in vx
                       //fx3x
                       3 => // bcd of vx in i,  i+1, i+2
                       {
                           mem[self.ar       as usize] = (vx / 100) % 10;
                           mem[(self.ar + 1) as usize] = (vx / 10)  % 10;
                           mem[(self.ar + 2) as usize] = vx         % 10;
                       }, 
                       5 => self.register_dump(mem, &(op[1] as i16)),
                       6 => self.register_load(mem, &(op[1] as i16)),
                       _ => (),
                   },
               _ => return,
           }
       }
    }
//}
