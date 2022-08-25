mod platform;

// mod cpu;
// mod flags;
mod cpu;
fn main() {
    let mut invaders = platform::InvadersCabinet::init(None);
    loop {
        invaders.emulate_cycle();
    }
}