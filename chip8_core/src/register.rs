const NUM_REGS: usize = 16;

pub(crate) struct RegisterSet([u8; NUM_REGS]);

impl Default for RegisterSet {
    fn default() -> Self {
        Self([0; NUM_REGS])
    }
}

impl std::ops::Deref for RegisterSet {
    type Target = [u8; NUM_REGS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RegisterSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
