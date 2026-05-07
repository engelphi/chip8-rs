#[derive(thiserror::Error, Debug)]
pub enum EmulationError {
    #[error("Invalid Opcode detected: {:04x}", .read_opcode)]
    InvalidOpcode { read_opcode: u16 },
}

pub type Result<T> = std::result::Result<T, EmulationError>;
