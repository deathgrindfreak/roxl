#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
    ValueError(&'static str),
}

#[derive(Debug)]
pub enum ChunkError {
    IPOutOfBoundsError,
    BadOPCodeError(u8),
}

impl From<ChunkError> for InterpretError {
    fn from(_value: ChunkError) -> InterpretError {
        InterpretError::RuntimeError
    }
}
