const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Screen([bool; SCREEN_WIDTH * SCREEN_HEIGHT]);

impl Default for Screen {
    fn default() -> Self {
        Self([false; SCREEN_WIDTH * SCREEN_HEIGHT])
    }
}
impl Screen {
    pub const fn width() -> usize {
        SCREEN_WIDTH
    }

    pub const fn height() -> usize {
        SCREEN_HEIGHT
    }

    pub fn clear(&mut self) {
        self.0 = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }
}

impl std::ops::Deref for Screen {
    type Target = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
