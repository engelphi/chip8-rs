const STACK_SIZE: usize = 16;

pub(crate) struct Stack {
    sp: u16,
    stack: [u16; STACK_SIZE],
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            sp: 0,
            stack: [0; STACK_SIZE],
        }
    }
}

impl Stack {
    pub(crate) fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub(crate) fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
