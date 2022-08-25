use crate::cpu::Cpu;
pub mod io;
use std::env;
use std::fs::File;
use std::io::Read;
pub struct InvadersCabinet {
    cpu: Cpu,
}

impl InvadersCabinet {

    // Pass None to InvadersCabinet::init to power on without loading a program

    pub fn init(path: Option<String>) -> Self {
        // Initialize the cpu and configure the actual arcade cabinet, such as setting up IO and interupts
        let mut buf: Vec<u8> = Vec::new();

        match path {
            Some(file_name) => {
                let args: Vec<String> = env::args().collect();
                let mut file = File::open(file_name).expect("File failed to read!");
                
                file.read_to_end(&mut buf).unwrap();
            }
            None => {},
        }

        let intel_8080 = Cpu::init(0x0, &buf);

        
        
        InvadersCabinet {
            cpu: intel_8080,
            
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.cpu.cycle_d();
    }

    // Fetches the framebuffer

    pub fn get_frame(&self) -> Vec<u8> {
        self.cpu.memory[0x2400..0x3fff].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn framebuffer_test() {
        let mut invaders = InvadersCabinet::init(None);
        let new_vram = [69; (0x3fff - 0x2400)];
        // invaders.cpu.memory[2400..0x3fff].copy_from_slice(&new_vram);
        invaders.cpu.memory[0x2400..0x3fff].copy_from_slice(&new_vram);
        assert_eq!(invaders.cpu.memory[0x2400..0x3fff], new_vram);
    }
}