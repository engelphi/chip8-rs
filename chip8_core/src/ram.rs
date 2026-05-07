const RAM_SIZE: usize = 4096;

pub(crate) struct RAM([u8; RAM_SIZE]);

impl Default for RAM {
    fn default() -> Self {
        Self([0; RAM_SIZE])
    }
}

impl std::ops::Deref for RAM {
    type Target = [u8; RAM_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RAM {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
