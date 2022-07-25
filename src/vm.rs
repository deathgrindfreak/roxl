use crate::chunk::{Chunk, OpCode, ChunkError};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl From<ChunkError> for InterpretError {
    fn from(_value: ChunkError) -> InterpretError {
        InterpretError::RuntimeError
    }
}

impl<'a> VM<'a> {
    pub fn instruct(&mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn read_op(&mut self) -> Result<OpCode, ChunkError> {
        let op = self.chunk.read_op(self.ip)?;
        self.ip += 1;
        Ok(op)
    }

    fn read_byte(&mut self) -> Result<u8, ChunkError> {
        let op = self.chunk.read(self.ip)?;
        self.ip += 1;
        Ok(op)
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            match self.read_op()? {
                OpCode::Return => { break; },
                OpCode::Constant => {
                    let constant = self.chunk.read_constant(self.read_byte()?.into())?;
                    println!("{}", constant);
                },
                OpCode::ConstantLong => {
                    let mut idx: usize = 0;
                    for _ in 0..=2 {
                        let b: usize = self.read_byte()?.into();
                        idx = (idx << 2) + b;
                    }

                    let constant = self.chunk.read_constant(idx)?;
                    println!("{}", constant);
                },
            };
        }
        Ok(())
    }
}
