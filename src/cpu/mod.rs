mod flags;

use crate::platform::io::IO;

use std::mem;
use flags::Flags;

const MEMORY: usize = 0xFA00;

const CYCLES: [u8; 256] = [
    //  0  1   2   3   4   5   6   7   8  9   A   B   C   D   E  F
        4, 10, 7,  5,  5,  5,  7,  4,  4, 10, 7,  5,  5,  5,  7, 4,  // 0
        4, 10, 7,  5,  5,  5,  7,  4,  4, 10, 7,  5,  5,  5,  7, 4,  // 1
        4, 10, 16, 5,  5,  5,  7,  4,  4, 10, 16, 5,  5,  5,  7, 4,  // 2
        4, 10, 13, 5,  10, 10, 10, 4,  4, 10, 13, 5,  5,  5,  7, 4,  // 3
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 4
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 5
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 6
        7, 7,  7,  7,  7,  7,  7,  7,  5, 5,  5,  5,  5,  5,  7, 5,  // 7
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // 8
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // 9
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // A
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // B
        5, 10, 10, 10, 11, 11, 7,  11, 5, 10, 10, 10, 11, 17, 7, 11, // C
        5, 10, 10, 10, 11, 11, 7,  11, 5, 10, 10, 10, 11, 17, 7, 11, // D
        5, 10, 10, 18, 11, 11, 7,  11, 5, 5,  10, 4,  11, 17, 7, 11, // E
        5, 10, 10, 4,  11, 11, 7,  11, 5, 5,  10, 4,  11, 17, 7, 11  // F
];

pub struct Registers {
    pub pc: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
}

impl Registers {
    fn set_bc(&mut self, value: u16) {
        self.c = (value & 0xff) as u8;
        self.b = ((value >> 8) & 0xff) as u8;
    }

    fn set_de(&mut self, value: u16) {
        self.e = (value & 0xff) as u8;
        self.d = ((value >> 8) & 0xff) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.l = (value & 0xff) as u8;
        self.h = ((value >> 8) & 0xff) as u8;
    }
}

pub struct Cpu {
    pub regs: Registers,

    pub flags: Flags,

    pub memory: [u8; MEMORY],

    pub hlted: bool,

    pub instructions: usize,

    immediate: [u8; 2],

    cycle: u8,

    io: IO,

    pub output: String,
}

impl Cpu {
    pub fn init(pc: u16, program: &[u8]) -> Cpu {
        let mut c = Cpu {

        
            regs: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,

                pc: 0,
                sp: 0,
            },

            cycle: 0,

            flags: Flags {
                sign: false,
                zero: false,
                parity: false,
                carry: false,
                aux_carry: false,
            },

            memory: [0; MEMORY],
            hlted: false,
            instructions: 0,
            immediate: [0, 0],

            output: String::new(),

            io: IO::default(),

        };

        c.load_into_memory(program, pc as usize);
        c
    }

    // Load array of bytes into memory

    pub fn load_into_memory(&mut self, bytes: &[u8], address: usize) -> Result<(), &'static str> {
        if bytes.len() == 0 {
            return Err("File is empty");
        }

        println!("CPU Loaded program to address 0x{:X}", address);

        for i in 0..bytes.len() {
            self.memory[i + address] = bytes[i];
            // println!("HEX DUMP: 0x{:X}", bytes[i]);
        }

        Ok(())
    }

    // Cycle with cycle durations
    pub fn cycle_d(&mut self) {
        if self.cycle > CYCLES[self.memory[self.regs.pc as usize] as usize] {
            self.cycle();
           self.cycle = 0;
        //    println!("SDF");
        } else {
            self.cycle += 1;
            // println!("UA");
        }

    }

    // Cycle the cpu once

    pub fn cycle(&mut self) {
        let mut advance = 1;
        self.instructions += 1;
        self.immediate = [self.memory[self.regs.pc as usize + 1], self.memory[self.regs.pc as usize + 2]];

        match self.memory[self.regs.pc as usize] {

            0x00 => {}

            /*
            DATA TRANSFER INSTRUCTIONS
            */


            // MOV INSTRUCTIONS
            0x40 => {},
            0x41 => self.regs.b = self.regs.c,
            0x42 => self.regs.b = self.regs.d,
            0x43 => self.regs.b = self.regs.e,
            0x44 => self.regs.b = self.regs.h,
            0x45 => self.regs.b = self.regs.l,
            0x46 => self.regs.b = self.get_m(),
            0x47 => self.regs.b = self.regs.a,
            0x48 => self.regs.c = self.regs.b,
            0x49 => {},
            0x4A => self.regs.c = self.regs.d,
            0x4B => self.regs.c = self.regs.e,
            0x4C => self.regs.c = self.regs.h,
            0x4D => self.regs.c = self.regs.l,
            0x4E => self.regs.c = self.get_m(),
            0x4F => self.regs.c = self.regs.a,
            0x50 => self.regs.d = self.regs.b,
            0x51 => self.regs.d = self.regs.c,
            0x52 => {},
            0x53 => self.regs.d = self.regs.e,
            0x54 => self.regs.d = self.regs.h,
            0x55 => self.regs.d = self.regs.l,
            0x56 => self.regs.d = self.get_m(),
            0x57 => self.regs.d = self.regs.a,
            0x58 => self.regs.e = self.regs.b,
            0x59 => self.regs.e = self.regs.c,
            0x5A => self.regs.e = self.regs.d,
            0x5B => {},
            0x5C => self.regs.e = self.regs.h,
            0x5D => self.regs.e = self.regs.l,
            0x5E => self.regs.e = self.get_m(),
            0x5F => self.regs.e = self.regs.a,
            0x60 => self.regs.h = self.regs.b,
            0x61 => self.regs.h = self.regs.c,
            0x62 => self.regs.h = self.regs.d,
            0x63 => self.regs.h = self.regs.e,
            0x64 => {},
            0x65 => self.regs.h = self.regs.l,
            0x66 => self.regs.h = self.get_m(),
            0x67 => self.regs.h = self.regs.a,
            0x68 => self.regs.l = self.regs.b,
            0x69 => self.regs.l = self.regs.c,
            0x6A => self.regs.l = self.regs.d,
            0x6B => self.regs.l = self.regs.e,
            0x6C => self.regs.l = self.regs.h,
            0x6D => {},
            0x6E => self.regs.l = self.get_m(),
            0x6F => self.regs.l = self.regs.a,
            0x70 => self.set_m(self.regs.b),
            0x71 => self.set_m(self.regs.c),
            0x72 => self.set_m(self.regs.d),
            0x73 => self.set_m(self.regs.e),
            0x74 => self.set_m(self.regs.h),
            0x75 => self.set_m(self.regs.l),
            0x77 => self.set_m(self.regs.a),
            0x78 => self.regs.a = self.regs.b,
            0x79 => self.regs.a = self.regs.c,
            0x7A => self.regs.a = self.regs.d,
            0x7B => self.regs.a = self.regs.e,
            0x7C => self.regs.a = self.regs.h,
            0x7D => self.regs.a = self.regs.l,
            0x7E => self.regs.a = self.get_m(),
            0x7F => {},

            /*
            MVI Instructions, move immediate value into register
            */
            0x06 => {self.regs.b = self.immediate[0]; advance = 2},
            0x16 => {self.regs.d = self.immediate[0]; advance = 2},
            0x26 => {self.regs.h = self.immediate[0]; advance = 2},
            0x36 => {self.memory[self.cmb_be(self.regs.h, self.regs.l) as usize] = self.immediate[0]; advance = 2},
            0x0E => {self.regs.c = self.immediate[0]; advance = 2},
            0x1E => {self.regs.e = self.immediate[0]; advance = 2},
            0x2E => {self.regs.l = self.immediate[0]; advance = 2},
            0x3E => {self.regs.a = self.immediate[0]; advance = 2},

            /*
            LXI Instructions
            */

            0x01 => {self.set_bc_imm(); advance = 3},
            0x11 => {self.set_de_imm(); advance = 3},
            0x21 => {self.set_hl_imm(); advance = 3},
            0x31 => {self.regs.sp = self.cmb_le(self.immediate[0], self.immediate[1]); advance = 3},
            
            // 50 % sure this works

            // STA and LDA
            0x32 => {self.mem_write(self.cmb_le(self.immediate[0], self.immediate[1]).into(), self.regs.a); advance = 3},
            0x3A => {self.regs.a = self.memory[self.cmb_le(self.immediate[0], self.immediate[1]) as usize]; advance = 3},

            // LHLD and SHLD
            0x2A => {
                self.regs.l = self.memory[self.cmb_le(self.immediate[0], self.immediate[1]) as usize] as u8;
                self.regs.h = self.memory[((self.cmb_le(self.immediate[0], self.immediate[1])) + 1) as usize] as u8;
                advance = 3;
            }

            0x22 => {
                self.mem_write(self.cmb_le(self.immediate[0], self.immediate[1]) as usize, self.regs.l);
                self.mem_write(((self.cmb_le(self.immediate[0], self.immediate[1])) + 1).into(), self.regs.h);
                advance = 3;
            }

            // LDAX
            0x0A => self.ldax(self.regs.b, self.regs.c),
            0x1A => self.ldax(self.regs.d, self.regs.e),

            // STAX

            0x02 => {
                
                self.mem_write(self.cmb_be(self.regs.b, self.regs.c) as usize, self.regs.a);
            }

            0x12 => {
                self.mem_write(self.cmb_be(self.regs.d, self.regs.e) as usize, self.regs.a);
            }

            // DAD
            0x09 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let bc32: u32 = self.cmb_be(self.regs.b, self.regs.c).into();
                let res: u32 = hl32 + bc32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            0x19 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let de32: u32 = self.cmb_be(self.regs.d, self.regs.e).into();
                let res: u32 = hl32 + de32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            0x29 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let de32: u32 = self.cmb_be(self.regs.b, self.regs.c).into();
                let res: u32 = hl32 + hl32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            // 0x32 => {
            //     println!("{}", self.cmb_le(self.immediate[0], self.immediate[1]));
            // }













            /*
            Branch instructions
            */

            0xc3 => {self.jmp_immediate(); advance = 0},

            0xCA => {self.jmp_if(self.flags.zero    == true,  &mut advance)}
            0xC2 => {self.jmp_if(self.flags.zero    == false, &mut advance)}
            0xD2 => {self.jmp_if(self.flags.carry   == false, &mut advance)}
            0xDA => {self.jmp_if(self.flags.carry   == true,  &mut advance)}
            0xE2 => {self.jmp_if(self.flags.parity  == false, &mut advance)}
            0xEA => {self.jmp_if(self.flags.parity  == true,  &mut advance)}
            0xF2 => {self.jmp_if(self.flags.sign    == false, &mut advance)}
            0xFA => {self.jmp_if(self.flags.sign    == true,  &mut advance)}

            0xCD => self.call_imm(&mut advance),
            0xC4 => self.call_if(self.flags.zero    == false, &mut advance),
            0xD4 => self.call_if(self.flags.carry   == false, &mut advance),
            0xE4 => self.call_if(self.flags.parity  == false, &mut advance),
            0xF4 => self.call_if(self.flags.sign    == false, &mut advance),
            0xCC => self.call_if(self.flags.zero    == true,  &mut advance),
            0xDC => self.call_if(self.flags.carry   == true,  &mut advance),
            0xEC => self.call_if(self.flags.parity  == true,  &mut advance),
            0xFC => self.call_if(self.flags.sign    == true,  &mut advance),


            0xC9 => {self.ret(); advance = 3},
            0xC8 => {self.ret_if(self.flags.zero == true, &mut advance)}
            0xD8 => {self.ret_if(self.flags.carry == true, &mut advance)}
            0xE8 => {self.ret_if(self.flags.parity == true, &mut advance)}
            0xF8 => {self.ret_if(self.flags.sign == true, &mut advance)}
            0xC0 => {self.ret_if(self.flags.zero == false, &mut advance)}
            0xD0 => {self.ret_if(self.flags.carry == false, &mut advance)}
            0xE0 => {self.ret_if(self.flags.parity == false, &mut advance)}
            0xF0 => {self.ret_if(self.flags.sign == false, &mut advance)}

            /*
            Math
            */

            0xC6 => {self.add(self.immediate[0]); advance = 2}
            0xCE => {self.add(self.immediate[0] + self.flags.carry as u8); advance = 2}

            0xD6 => {self.sub(self.immediate[0]); advance = 2}
            0xDE => {self.sub(self.immediate[0] + self.flags.carry as u8); advance = 2}

            0x80 => self.add(self.regs.b),
            0x81 => self.add(self.regs.c),
            0x82 => self.add(self.regs.d),
            0x83 => self.add(self.regs.e),
            0x84 => self.add(self.regs.h),
            0x85 => self.add(self.regs.l),
            0x86 => self.add(self.get_m()),
            0x87 => self.add(self.regs.a),
            0x88 => self.add(self.regs.b + (self.flags.carry as u8)),
            0x89 => self.add(self.regs.c + (self.flags.carry as u8)),
            0x8A => self.add(self.regs.d + (self.flags.carry as u8)),
            0x8B => self.add(self.regs.e + (self.flags.carry as u8)),
            0x8C => self.add(self.regs.h + (self.flags.carry as u8)),
            0x8D => self.add(self.regs.l + (self.flags.carry as u8)),
            0x8E => self.add(self.get_m() + (self.flags.carry as u8)),
            0x8F => self.add(self.regs.a + (self.flags.carry as u8)),


            0x90 => self.sub(self.regs.b),
            0x91 => self.sub(self.regs.c),
            0x92 => self.sub(self.regs.d),
            0x93 => self.sub(self.regs.e),
            0x94 => self.sub(self.regs.h),
            0x95 => self.sub(self.regs.l),
            0x96 => self.sub(self.get_m()),
            0x97 => self.sub(self.regs.a),
            0x98 => self.sub(self.regs.b + (self.flags.carry as u8)),
            0x99 => self.sub(self.regs.c + (self.flags.carry as u8)),
            0x9A => self.sub(self.regs.d + (self.flags.carry as u8)),
            0x9B => self.sub(self.regs.e + (self.flags.carry as u8)),
            0x9C => self.sub(self.regs.h + (self.flags.carry as u8)),
            0x9D => self.sub(self.regs.l + (self.flags.carry as u8)),
            0x9E => self.sub(self.get_m() + (self.flags.carry as u8)),
            0x9F => self.sub(self.regs.a + (self.flags.carry as u8)),
            

            0xE6 => {self.ani(); advance = 2}
            0xA0 => self.ana(self.regs.b),
            0xA1 => self.ana(self.regs.c),
            0xA2 => self.ana(self.regs.d),
            0xA3 => self.ana(self.regs.e),
            0xA4 => self.ana(self.regs.h),
            0xA5 => self.ana(self.regs.l),
            0xA6 => self.ana(self.get_m()),
            0xA7 => self.ana(self.regs.a),


            0xF6 => {self.ori(); advance = 2}
            0xB0 => self.ora(self.regs.b),
            0xB1 => self.ora(self.regs.c),
            0xB2 => self.ora(self.regs.d),
            0xB3 => self.ora(self.regs.e),
            0xB4 => self.ora(self.regs.h),
            0xB5 => self.ora(self.regs.l),
            0xB6 => self.ora(self.get_m()),
            0xB7 => self.ora(self.regs.a),

            0xEE => {self.xri(); advance = 2}

            0x04 => self.regs.b = self.inr(self.regs.b),
            0x0C => self.regs.c = self.inr(self.regs.c),
            0x14 => self.regs.d = self.inr(self.regs.d),
            0x1C => self.regs.e = self.inr(self.regs.e),
            0x24 => self.regs.h = self.inr(self.regs.h),
            0x2C => self.regs.l = self.inr(self.regs.l),
            0x34 => { let m = self.get_m(); let res = self.inr(m); self.set_m(res); }
            0x3C => self.regs.a = self.inr(self.regs.a),
            0x05 => self.regs.b = self.dcr(self.regs.b),
            0x0D => self.regs.c = self.dcr(self.regs.c),
            0x15 => self.regs.d = self.dcr(self.regs.d),
            0x1D => self.regs.e = self.dcr(self.regs.e),
            0x25 => self.regs.h = self.dcr(self.regs.h),
            0x2D => self.regs.l = self.dcr(self.regs.l),
            0x35 => { let m = self.get_m(); let res = self.dcr(m); self.set_m(res); }
            0x3D => self.regs.a = self.dcr(self.regs.a),

            // INX and DCX

            0x03 => self.regs.set_bc(self.cmb_be(self.regs.b, self.regs.c) + 1),
            0x13 => self.regs.set_de(self.cmb_be(self.regs.d, self.regs.e) + 1),
            0x23 => self.regs.set_hl(self.cmb_be(self.regs.h, self.regs.l) + 1),
            0x33 => self.regs.sp += 1,
            0x0B => self.regs.set_bc(self.cmb_be(self.regs.b, self.regs.c) - 1),
            0x1B => self.regs.set_de(self.cmb_be(self.regs.d, self.regs.e) - 1),
            0x2B => self.regs.set_hl(self.cmb_be(self.regs.h, self.regs.l) - 1),
            0x3B => self.regs.sp -= 2,

            /*
            Stack Functions
            */

            0xC5 => self.push_regs(self.regs.b, self.regs.c),
            0xD5 => self.push_regs(self.regs.d, self.regs.e),
            0xE5 => self.push_regs(self.regs.h, self.regs.l),
            
            0xC1 => self.pop_into_bc(),
            0xD1 => self.pop_into_de(),
            0xE1 => self.pop_into_hl(),

            0xF1 => {
                self.regs.a = self.memory[(self.regs.sp + 1) as usize];
                let psw: u8 = self.memory[self.regs.sp as usize];
                self.flags.zero = (0x01 == (psw & 0x01));
                self.flags.sign = (0x02 == (psw & 0x02));
                self.flags.parity = (0x04 == (psw & 0x04));
                self.flags.carry =  (0x05 == (psw & 0x08));
                self.flags.aux_carry = (0x10 == (psw & 0x10));
                self.regs.sp += 2;

            }

            0xf5 => {
                self.mem_write((self.regs.sp - 1) as usize, self.regs.a);
                let psw: u8 = (self.flags.zero as u8) | ((self.flags.sign as u8) << 1) | 
                ((self.flags.parity as u8) << 2) | ((self.flags.carry as u8) << 3) | ((self.flags.aux_carry as u8) << 4);
                self.mem_write((self.regs.sp - 2) as usize, psw);
                self.regs.sp = self.regs.sp - 2;
            }

            // Logical

            0xFE => {self.cmp(self.immediate[0].into()); advance = 2}
            0xB8 => self.cmp(self.regs.b.into()),
            0xB9 => self.cmp(self.regs.c.into()),
            0xBA => self.cmp(self.regs.d.into()),
            0xBB => self.cmp(self.regs.e.into()),
            0xBC => self.cmp(self.regs.h.into()),
            0xBD => self.cmp(self.regs.l.into()),
            0xBF => self.cmp(self.regs.a.into()),
            0xBE => self.cmp(self.get_m().into()),

            0xEB => {mem::swap(&mut self.regs.h, &mut self.regs.d); mem::swap(&mut self.regs.l, &mut self.regs.e);}
            

            0xA8 => self.xra(self.regs.b),
            0xA9 => self.xra(self.regs.c),
            0xAA => self.xra(self.regs.d),
            0xAB => self.xra(self.regs.e),
            0xAC => self.xra(self.regs.h),
            0xAD => self.xra(self.regs.l),
            0xAE => self.xra(self.get_m()),
            0xAF => self.xra(self.regs.a),

            0x0F => {
                let x: u8 = self.regs.a;
                self.regs.a = ((x & 1) << 7) | (x >> 1);
                self.flags.carry = (1 == (x&1));
            }

            /*
            IO Instructions 
            */

            0xD3 => {
                // todo!();
                // (self.in_routine)(4);
                // self.io.
                self.io.cpu_write(self.immediate[0], self.regs.a);
                // self.i
                advance = 2},
            0xFB => {},
            
            /*
            MISC Instructions
            */
            
            // ENABLE CFLAG
            0x37 => self.flags.carry = true,
            // DISABLE CFLAG
            0x3F => self.flags.carry = false,

            // CMA, Compliment A
            0x2F => self.regs.a = !self.regs.a,

            0x76 => self.hlted = true,

            
            

            _ => unimplemented!("Opcode 0x{:X}", self.memory[self.regs.pc as usize]),
            // _ => unimplemented!(),
        }

        self.regs.pc += advance;
    }

    pub fn debug(&self) {
        println!("OP 0x{:X}", self.memory[self.regs.pc as usize]);
        // println!("{:X}", self.memory[0x1000]);
        println!("A {:X}", self.regs.pc);
    }

    fn set_bc_imm(&mut self) {
        self.regs.b = self.immediate[1];
        self.regs.c = self.immediate[0];
    }

    fn set_de_imm(&mut self) {
        self.regs.d = self.immediate[1];
        self.regs.e = self.immediate[0];
    }

    fn set_hl_imm(&mut self) {
        self.regs.h = self.immediate[1];
        self.regs.l = self.immediate[0];
    }



    fn jmp_if(&mut self, condition: bool, advance: *mut u16) {
        if (condition == true) {
            self.jmp_immediate();
            unsafe {
                *advance = 0;
            }
        } else {
            unsafe {
                *advance = 3;
            }
        }
    }

    fn call_if(&mut self, condition: bool, advance: *mut u16) {
        if (condition == true) {
            self.call_imm(advance);
        } else {
            unsafe {
                *advance = 3;
            }
        }
    }

    fn jmp_immediate(&mut self) {
        self.regs.pc = self.cmb_le(self.immediate[0], self.immediate[1]);
    }

    fn ret(&mut self) {
        self.regs.pc = self.cmb_le(self.memory[self.regs.sp as usize], self.memory[(self.regs.sp + 1) as usize]);

        self.regs.sp += 2;
    }

    fn ret_if(&mut self, condition: bool, advance: *mut u16) {
        if condition == true {
            self.ret();
            unsafe {
                *advance = 3;
            }
        } else {
            unsafe {
                *advance = 1;
            }
        }
    }

    fn push_regs(&mut self, reg1: u8, reg2: u8) {
        self.mem_write((self.regs.sp - 2) as usize, reg2);
        self.mem_write((self.regs.sp - 1) as usize, reg1);
        self.regs.sp -= 2;
    }

    fn pop_into_bc(&mut self) {
        self.regs.c = self.memory[self.regs.sp as usize];
        self.regs.b = self.memory[(self.regs.sp + 1) as usize];
        self.regs.sp += 2;
    }

    fn pop_into_de(&mut self) {
        self.regs.e = self.memory[self.regs.sp as usize];
        self.regs.d = self.memory[(self.regs.sp + 1) as usize];
        self.regs.sp += 2;
    }

    fn pop_into_hl(&mut self) {
        self.regs.l = self.memory[self.regs.sp as usize];
        self.regs.h = self.memory[(self.regs.sp + 1) as usize];
        self.regs.sp += 2;
    }

    // Normal CPU Call
    
    fn nm_call(&mut self) {
        self.mem_write((self.regs.sp - 1) as usize, ((self.regs.pc >> 8) & 0xff) as u8);
        self.mem_write((self.regs.sp - 2) as usize, (self.regs.pc & 0xff) as u8);

        self.regs.sp -= 2;

        self.jmp_immediate();
    }

    // Call with CP\M Support

    fn cpm_call(&mut self, advance: *mut u16) {
        const BDOS: u16 = 5;

        const WRITESTR: u8 = 9;

        let addr = self.cmb_le(self.immediate[0], self.immediate[1]);

        if addr == BDOS {
            if self.regs.c == WRITESTR {
                let string_addr = self.cmb_be(self.regs.d, self.regs.e);
                let mut c = 0;
                while self.memory[(string_addr + c) as usize] != '$' as u8 {
                    print!("{}", self.memory[(string_addr + c) as usize] as char);
                    self.output.push(self.memory[(string_addr + c) as usize] as char);
                    c += 1;
                }
            }

            unsafe {
                *advance = 3;
            }
            
        } else {
            self.nm_call();
            unsafe {
                *advance = 0;
            }
        }
    }

    fn call_imm(&mut self, advance: *mut u16) {
        #[cfg(not(feature = "cpm"))]
        {
            self.nm_call();
            unsafe {
                *advance = 0;
            }
        }

        #[cfg(feature = "cpm")]
        {
            // Add --features cpm to your cargo build arguments to include CP\M functionality
            self.cpm_call(advance);
        }
    }

    pub fn cmb_le(&self, val1: u8, val2: u8) -> u16 {
        (val2 as u16) << 8 | val1 as u16
    }

    pub fn cmb_be(&self, val1: u8, val2: u8) -> u16 {
        (val1 as u16) << 8 | val2 as u16
    }

    fn mem_write(&mut self, pos: usize, val: u8) {
        self.memory[pos] = val;
    }

    /* 
    Arithmitic instructions
    */

    fn ani(&mut self) {
        let res = (self.regs.a & self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = (self.regs.a & self.immediate[0]) as u8
    }

    fn ana(&mut self, val: u8) {
        let res = (self.regs.a & val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = (self.regs.a & val) as u8
    }

    fn ori(&mut self) {
        let res = (self.regs.a | self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }


    fn ora(&mut self, val: u8) {
        let res = (self.regs.a | val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        self.regs.a = res as u8;
    }

    fn xri(&mut self) {
        let res = (self.regs.a ^ self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }

    fn xra(&mut self, val: u8) {
        let res = (self.regs.a ^ val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }

    fn add(&mut self, val: u8) {
        let res: u16 = ((self.regs.a as u16) + val as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        self.regs.a = res as u8;
    }

    fn inr(&mut self, reg: u8) -> u8 {
        let res: u16 = ((reg as u16) + 1 as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        res as u8
        
    }



    // Returns add but with setting flags

    fn get_add(&mut self, val: u16) -> u8 {
        let res: u16 = ((self.regs.a as u16) + val as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);

        return res as u8;
    }

    fn sub(&mut self, val: u8) {
        let r1 = (self.regs.a as u16).wrapping_sub(val as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        self.regs.a = r1 as u8;
        // println!("{}", self.flags.carry);
        // i8080_set_flag(cpu, FLAG_C, !(res16 & 0x100));
    }

    fn get_sub(&mut self, val: u8) -> u8 {
        let r1 = (self.regs.a as u16).wrapping_sub(val as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        r1 as u8
    }

    fn dcr(&mut self, reg: u8) -> u8 {
        let r1 = (reg as u16).wrapping_sub(1 as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        r1 as u8
    }
    
    fn set_m(&mut self, val: u8) {
        self.mem_write((self.cmb_be(self.regs.h, self.regs.l) as usize) as usize, val);
    }

    fn get_m(&self) -> u8 {
        self.memory[self.cmb_be(self.regs.h, self.regs.l) as usize]
    }

    // DATA TRANSFER

    fn ldax(&mut self, reg1: u8, reg2: u8) {
        self.regs.a = self.memory[self.cmb_be(reg1, reg2) as usize];
    }

    // Logical

    fn cmp(&mut self, val: u16) {
        // let res: u16 = (self.regs.a as u16) - val as u16;
        let res: u16 = (self.regs.a as u16).wrapping_sub(val as u16);
        // let
        self.flags.zero = res == 0;
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.carry = (self.regs.a < val as u8);
    }

    // Communication with the outside machine/emulator

    // Return list of registers with corrosponding names

    pub fn get_regs(&self) -> ([&str; 9], [u8; 7], [u16; 2]) {
        let regs = [
            self.regs.a,
            self.regs.b,
            self.regs.c,
            self.regs.d,
            self.regs.e,
            self.regs.h,
            self.regs.l,
        ];

        let regs16 = [self.regs.pc, self.regs.sp];

        let names = [
            "A",
            "B",
            "C",
            "D",
            "E",
            "H",
            "L",
            "PC",
            "SP",
        ];

        (names, regs, regs16)
    }







}

#[cfg(test)]
mod cpu_test {
    use super::*;

    #[test]
    fn lxi_bc() {
        let prog = [0x01, 0x34, 0x12];
        // LXI B, 0x1234
        let mut cpu = Cpu::init(0x0, &prog);
        cpu.cycle();

        assert_eq!(cpu.regs.b, 0x12);
        assert_eq!(cpu.regs.c, 0x34);

    }

    #[test]
    fn lxi_de() {
        let prog = [0x11, 0x34, 0x12];
        // LXI B, 0x1234
        let mut cpu = Cpu::init(0x0, &prog);
        cpu.cycle();

        assert_eq!(cpu.regs.d, 0x12);
        assert_eq!(cpu.regs.e, 0x34);

    }

    #[test]
    fn lxi_hl() {
        let prog = [0x21, 0x34, 0x12];
        // LXI B, 0x1234
        let mut cpu = Cpu::init(0x0, &prog);
        cpu.cycle();

        assert_eq!(cpu.regs.h, 0x12);
        assert_eq!(cpu.regs.l, 0x34);

    }

    #[test]
    fn set_and_read_m() {
        let prog = [0x21, 0x00, 0x80, 0x36, 0xff];
        // Prog in assembly is:

        /*

        lxi h, 0x8000

        mvi m, 0xff

        */
        
        
        let mut cpu = Cpu::init(0x0, &prog);
        for i in 0..2 {
            cpu.cycle();
        }

        
        assert_eq!(cpu.memory[0x8000], 0xff);
    }
}