use crate::{EmulationError, Result};
pub(crate) enum OpCode {
    NoOp,
    ClearScreen,
    ReturnFromSubroutine,
    Jump { addr: u16 },
    CallSubroutine { addr: u16 },
    SkipNextIfVXEqNN { vx: usize, nn: u8 },
    SkipNextIfVxNotEqNN { vx: usize, nn: u8 },
    SkipNextIfVxEqVy { vx: usize, vy: usize },
    SetVxToNN { vx: usize, nn: u8 },
    VxPlusNN { vx: usize, nn: u8 },
    SetVxToVy { vx: usize, vy: usize },
    VxBitwiseOrVy { vx: usize, vy: usize },
    VxBitwiseAndVy { vx: usize, vy: usize },
    VxBitwiseXorVy { vx: usize, vy: usize },
    VxPlusVy { vx: usize, vy: usize },
    VxMinusVy { vx: usize, vy: usize },
    VxShiftRightOne { vx: usize },
    SetVxToVyMinusVx { vx: usize, vy: usize },
    VxShiftLeftOne { vx: usize },
    SkipNextIfVxNotEqVy { vx: usize, vy: usize },
    SetIToNNN { nnn: u16 },
    JumpToV0PlusNNN { nnn: u16 },
    SetVxToRandBitwiseAndNN { vx: usize, nn: u8 },
    DrawSpriteAtVxVyForNRows { vx: usize, vy: usize, num_rows: u16 },
    SkipIfKeyPressed { vx: usize },
    SkipIfKeyNotPressed { vx: usize },
    SetVxToDt { vx: usize },
    WaitForKeyPress { vx: usize },
    SetDtToVx { vx: usize },
    SetStToVx { vx: usize },
    IPlusVx { vx: usize },
    SetIToFontAddress { vx: usize },
    SetIToBCDOfVx { vx: usize },
    StoreV0ToVxIntoI { vx: usize },
    LoadIIntoV0ToVx { vx: usize },
}

impl OpCode {
    pub(crate) fn decode(op: u16) -> Result<OpCode> {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => Ok(Self::NoOp),
            (0, 0, 0xE, 0) => Ok(Self::ClearScreen),
            (0, 0, 0xE, 0xE) => Ok(Self::ReturnFromSubroutine),
            (1, _, _, _) => Ok(Self::Jump { addr: op & 0x0FFF }),
            (2, _, _, _) => Ok(Self::CallSubroutine { addr: op & 0x0FFF }),
            (3, _, _, _) => Ok(Self::SkipNextIfVXEqNN {
                vx: digit2 as usize,
                nn: (op & 0x00FF) as u8,
            }),
            (4, _, _, _) => Ok(Self::SkipNextIfVxNotEqNN {
                vx: digit2 as usize,
                nn: (op & 0x00FF) as u8,
            }),
            (5, _, _, 0) => Ok(Self::SkipNextIfVxEqVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (6, _, _, _) => Ok(Self::SetVxToNN {
                vx: digit2 as usize,
                nn: (op & 0x00FF) as u8,
            }),
            (7, _, _, _) => Ok(Self::VxPlusNN {
                vx: digit2 as usize,
                nn: (op & 0x00FF) as u8,
            }),
            (8, _, _, 0) => Ok(Self::SetVxToVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 1) => Ok(Self::VxBitwiseOrVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 2) => Ok(Self::VxBitwiseAndVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 3) => Ok(Self::VxBitwiseXorVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 4) => Ok(Self::VxPlusVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 5) => Ok(Self::VxMinusVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 6) => Ok(Self::VxShiftRightOne {
                vx: digit2 as usize,
            }),
            (8, _, _, 7) => Ok(Self::SetVxToVyMinusVx {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (8, _, _, 0xE) => Ok(Self::VxShiftLeftOne {
                vx: digit2 as usize,
            }),
            (9, _, _, 0) => Ok(Self::SkipNextIfVxNotEqVy {
                vx: digit2 as usize,
                vy: digit3 as usize,
            }),
            (0xA, _, _, _) => Ok(Self::SetIToNNN { nnn: op & 0x0FFF }),
            (0xB, _, _, _) => Ok(Self::JumpToV0PlusNNN { nnn: op & 0x0FFF }),
            (0xC, _, _, _) => Ok(Self::SetVxToRandBitwiseAndNN {
                vx: digit2 as usize,
                nn: (op & 0x00FF) as u8,
            }),
            (0xD, _, _, _) => Ok(Self::DrawSpriteAtVxVyForNRows {
                vx: digit2 as usize,
                vy: digit3 as usize,
                num_rows: digit4 as u16,
            }),
            (0xE, _, 9, 0xE) => Ok(Self::SkipIfKeyPressed {
                vx: digit2 as usize,
            }),
            (0xE, _, 0xA, 1) => Ok(Self::SkipIfKeyNotPressed {
                vx: digit2 as usize,
            }),
            (0xF, _, 0, 7) => Ok(Self::SetVxToDt {
                vx: digit2 as usize,
            }),
            (0xF, _, 0, 0xA) => Ok(Self::WaitForKeyPress {
                vx: digit2 as usize,
            }),
            (0xF, _, 1, 5) => Ok(Self::SetDtToVx {
                vx: digit2 as usize,
            }),
            (0xF, _, 1, 8) => Ok(Self::SetStToVx {
                vx: digit2 as usize,
            }),
            (0xF, _, 1, 0xE) => Ok(Self::IPlusVx {
                vx: digit2 as usize,
            }),
            (0xF, _, 2, 9) => Ok(Self::SetIToFontAddress {
                vx: digit2 as usize,
            }),
            (0xF, _, 3, 3) => Ok(Self::SetIToBCDOfVx {
                vx: digit2 as usize,
            }),
            (0xF, _, 5, 5) => Ok(Self::StoreV0ToVxIntoI {
                vx: digit2 as usize,
            }),
            (0xF, _, 6, 5) => Ok(Self::LoadIIntoV0ToVx {
                vx: digit2 as usize,
            }),
            (_, _, _, _) => Err(EmulationError::InvalidOpcode { read_opcode: op }),
        }
    }
}
