use chip8_core::*;

#[cxx::bridge(namespace = "chip8")]
mod ffi {
    extern "Rust" {
        type Chip8Emu;
        #[Self = "Chip8Emu"]
        fn create() -> Box<Chip8Emu>;
        fn tick(&mut self);
        fn tick_timers(&mut self);
        fn reset(&mut self);
        fn load_game(&mut self, data: Vec<u8>);
        fn keypress(&mut self, key: &str, pressed: bool);
        fn screen_data(&self) -> &[bool];
        #[Self = "Chip8Emu"]
        fn screen_width() -> usize;
        #[Self = "Chip8Emu"]
        fn screen_height() -> usize;
    }
}

pub struct Chip8Emu(Chip8Core);

impl Chip8Emu {
    fn create() -> Box<Chip8Emu> {
        Box::new(Chip8Emu(Chip8Core::new()))
    }

    pub fn screen_width() -> usize {
        Screen::width()
    }

    pub fn screen_height() -> usize {
        Screen::height()
    }

    pub fn screen_data(&self) -> &[bool] {
        self.0.get_display()
    }

    pub fn tick(&mut self) {
        self.0.tick().expect("Failed to run next instruction");
    }

    pub fn tick_timers(&mut self) {
        self.0.tick_timers();
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn load_game(&mut self, data: Vec<u8>) {
        self.0.load(&data);
    }

    pub fn keypress(&mut self, key: &str, pressed: bool) {
        if let Some(k) = key2btn(&key) {
            self.0.keypress(k, pressed);
        }
    }
}

fn key2btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
