#[derive(Default)]
pub struct IO {
    shift_offset: u8,
    shift_reg: u16,

    // read_ports: [u8; 3],
    read_1: u8,
}

impl IO {
    pub fn cpu_write(&mut self, port: u8, value: u8) {
        match port {
            2 => self.shift_offset = value & 0x7,
            4 => self.shift_reg = (self.shift_reg >> 8) | (value as u16) << 8,
            6 => {},
            _ => unimplemented!(),
        }
    }

    pub fn cpu_read(&self, port: u8) -> u8 {
        match port {
            1 => self.read_1,
            _ => todo!(),
        }
    }

    pub fn set_port_1(&mut self, bit: u8) {
        self.read_1 |= 1 << bit;
    }

    pub fn insert_coin(&mut self) {
        self.set_port_1(0);
    }
}

#[cfg(test)]
mod io_test {
    use super::*;

    #[test]
    fn cpu_write_port_4() {
        let mut io = IO::default();
        io.cpu_write(4, 0xAB);
        assert_eq!(io.shift_reg, 0xAB00);

        io.cpu_write(4, 0xCD);
        assert_eq!(io.shift_reg, 0xCDAB);
    }

    #[test]
    fn cpu_read_port_1() {
        let mut io = IO::default();
        io.insert_coin();

        assert_eq!(io.read_1, 0b000001);

        
    }
}