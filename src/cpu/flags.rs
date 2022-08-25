const parity_table: [u8; 256] = [
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
    1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1
];
#[derive(Default)]
pub struct Flags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

impl Flags {
    // Checks msb (value that WILL be the sign flag)
    pub fn calc_sign(&self, res: u8) -> bool {
        ((res >> 7) & 1) == 1
    }

    pub fn calc_zero(&self, res: u8) -> bool {
        res == 0
    }

    // Calculates addition carry flag

    pub fn calc_add_cy(&self, res: u16) -> bool {
        res > 0xff
    }

    // Calculate subtraction carry flag

    pub fn calc_sub_cy(&self, res: u16) -> bool {
        todo!();
    }

    // Calculates parity flag
    // Cycles through the byte and checks if the number of bytes are even

    pub fn calc_parity(&self, res: u8) -> bool {
        let mut num_of_on_bits: u8 = 0;
        for i in 0..7 {
            let bit = ((res >> i) & 1) == 1;
            if bit == true {
                num_of_on_bits += 1;
            }
        }

        // num_of_on_bits % 2 == 0

        // 3 % 2 == 0
        parity_table[res as usize] == 1
    }

    /*
    Check if bit 4 of the result is the same as the accumulator, if not
    we will set the aux carry flag
    */

    pub fn calc_aux(&self, res: u16, accu: u8) -> bool {
        let b4 = ((res >> 4) & 1) as u8;
        let a_b4 = (accu >> 4) & 1;


        b4 != a_b4
   
    }

    pub fn set_all_flags(&mut self, res: u8) {
        self.sign = self.calc_sign(res);
        self.zero = self.calc_zero(res);
        
    }
}