const NUM_KEYS: usize = 16;

#[derive(Debug)]
pub(crate) struct Keys([bool; NUM_KEYS]);

impl std::ops::Deref for Keys {
    type Target = [bool; NUM_KEYS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Keys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self([false; NUM_KEYS])
    }
}
