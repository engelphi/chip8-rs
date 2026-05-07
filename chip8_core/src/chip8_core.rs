use crate::error::Result;
use crate::fontset::FONTSET;
use crate::keys::Keys;
use crate::opcode::OpCode;
use crate::ram::RAM;
use crate::register::RegisterSet;
use crate::screen::Screen;
use crate::stack::Stack;
use crate::timer::Timer;

const PC_START_ADDR: u16 = 0x200;

pub struct Chip8Core {
    pc: u16,
    ram: RAM,
    screen_buffer: Screen,
    v_regs: RegisterSet,
    i_reg: u16,
    stack: Stack,
    keys: Keys,
    dt: Timer,
    st: Timer,
}

impl Default for Chip8Core {
    fn default() -> Self {
        Self {
            pc: PC_START_ADDR,
            ram: Default::default(),
            screen_buffer: Default::default(),
            v_regs: Default::default(),
            i_reg: 0,
            stack: Default::default(),
            keys: Default::default(),
            dt: Default::default(),
            st: Default::default(),
        }
    }
}

impl Chip8Core {
    pub fn new() -> Self {
        let mut new_emu = Self::default();
        new_emu.ram[..FONTSET.len()].copy_from_slice(&FONTSET);
        new_emu
    }

    pub fn reset(&mut self) {
        *self = Self::default();
        self.ram[..FONTSET.len()].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, val: u16) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> u16 {
        self.stack.pop()
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (high_byte << 8) | low_byte;
        self.pc += 2;
        op
    }

    pub fn get_display(&self) -> &[bool] {
        &*self.screen_buffer
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = PC_START_ADDR as usize;
        let end = (PC_START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    pub fn tick(&mut self) -> Result<()> {
        let op = self.fetch();
        self.execute(OpCode::decode(op)?);
        Ok(())
    }

    fn execute(&mut self, op: OpCode) {
        match op {
            OpCode::NoOp => (),
            OpCode::ClearScreen => self.screen_buffer.clear(),
            OpCode::ReturnFromSubroutine => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            OpCode::Jump { addr } => self.pc = addr,
            OpCode::CallSubroutine { addr } => {
                self.push(self.pc);
                self.pc = addr;
            }
            OpCode::SkipNextIfVXEqNN { vx, nn } => {
                if self.v_regs[vx] == nn {
                    self.pc += 2;
                }
            }
            OpCode::SkipNextIfVxNotEqNN { vx, nn } => {
                if self.v_regs[vx] != nn {
                    self.pc += 2;
                }
            }
            OpCode::SkipNextIfVxEqVy { vx, vy } => {
                if self.v_regs[vx] == self.v_regs[vy] {
                    self.pc += 2;
                }
            }
            OpCode::SetVxToNN { vx, nn } => {
                self.v_regs[vx] = nn;
            }
            OpCode::VxPlusNN { vx, nn } => {
                self.v_regs[vx] = self.v_regs[vx].wrapping_add(nn);
            }
            OpCode::SetVxToVy { vx, vy } => {
                self.v_regs[vx] = self.v_regs[vy];
            }
            OpCode::VxBitwiseOrVy { vx, vy } => {
                self.v_regs[vx] |= self.v_regs[vy];
            }
            OpCode::VxBitwiseAndVy { vx, vy } => {
                self.v_regs[vx] &= self.v_regs[vy];
            }
            OpCode::VxBitwiseXorVy { vx, vy } => {
                self.v_regs[vx] ^= self.v_regs[vy];
            }
            OpCode::VxPlusVy { vx, vy } => {
                let (new_vx, carry) = self.v_regs[vx].overflowing_add(self.v_regs[vy]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_regs[vx] = new_vx;
                self.v_regs[0xF] = new_vf;
            }
            OpCode::VxMinusVy { vx, vy } => {
                let (new_vx, borrow) = self.v_regs[vx].overflowing_sub(self.v_regs[vy]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_regs[vx] = new_vx;
                self.v_regs[0xF] = new_vf;
            }
            OpCode::VxShiftRightOne { vx } => {
                let lsb = self.v_regs[vx] & 1;
                self.v_regs[vx] >>= 1;
                self.v_regs[0xF] = lsb;
            }
            OpCode::SetVxToVyMinusVx { vx, vy } => {
                let (new_vx, borrow) = self.v_regs[vy].overflowing_sub(self.v_regs[vx]);
                let new_vf = if borrow { 0 } else { 1 };
                self.v_regs[vx] = new_vx;
                self.v_regs[0xF] = new_vf;
            }
            OpCode::VxShiftLeftOne { vx } => {
                let msb = (self.v_regs[vx] >> 7) & 1;
                self.v_regs[vx] <<= 1;
                self.v_regs[0xF] = msb;
            }
            OpCode::SkipNextIfVxNotEqVy { vx, vy } => {
                if self.v_regs[vx] != self.v_regs[vy] {
                    self.pc += 2;
                }
            }
            OpCode::SetIToNNN { nnn } => self.i_reg = nnn,
            OpCode::JumpToV0PlusNNN { nnn } => self.pc = (self.v_regs[0] as u16) + nnn,
            OpCode::SetVxToRandBitwiseAndNN { vx, nn } => {
                let rng: u8 = rand::random();
                self.v_regs[vx] = rng & nn;
            }
            OpCode::DrawSpriteAtVxVyForNRows { vx, vy, num_rows } => {
                let x_coord = self.v_regs[vx] as u16;
                let y_coord = self.v_regs[vy] as u16;
                let mut flipped = false;
                for y_line in 0..num_rows {
                    let addr = (self.i_reg + y_line as u16) as usize;
                    let pixels = self.ram[addr];
                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % Screen::width();
                            let y = (y_coord + y_line) as usize % Screen::height();

                            let idx = x + Screen::width() * y;
                            flipped |= self.screen_buffer[idx];
                            self.screen_buffer[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_regs[0xF] = 1;
                } else {
                    self.v_regs[0xF] = 0;
                }
            }
            OpCode::SkipIfKeyPressed { vx } => {
                if self.keys[self.v_regs[vx] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfKeyNotPressed { vx } => {
                if !self.keys[self.v_regs[vx] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SetVxToDt { vx } => self.v_regs[vx] = *self.dt,
            OpCode::WaitForKeyPress { vx } => {
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_regs[vx] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }
            OpCode::SetDtToVx { vx } => {
                *self.dt = self.v_regs[vx];
            }
            OpCode::SetStToVx { vx } => {
                *self.st = self.v_regs[vx];
            }
            OpCode::IPlusVx { vx } => {
                self.i_reg = self.i_reg.wrapping_add(self.v_regs[vx] as u16);
            }
            OpCode::SetIToFontAddress { vx } => {
                let c = self.v_regs[vx] as u16;
                self.i_reg = c * 5;
            }
            OpCode::SetIToBCDOfVx { vx } => {
                let vx = self.v_regs[vx] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }
            OpCode::StoreV0ToVxIntoI { vx } => {
                let i = self.i_reg as usize;
                for idx in 0..=vx {
                    self.ram[i + idx] = self.v_regs[idx];
                }
            }
            OpCode::LoadIIntoV0ToVx { vx } => {
                let i = self.i_reg as usize;
                for idx in 0..=vx {
                    self.v_regs[idx] = self.ram[i + idx];
                }
            }
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }
}
