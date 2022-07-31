use crate::chunk::{Chunk, OpCode, ChunkError, Value};

#[derive(Default)]
pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

pub struct InterpretResult;

impl From<ChunkError> for InterpretError {
    fn from(_value: ChunkError) -> InterpretError {
        InterpretError::RuntimeError
    }
}

impl<'a> VM<'a> {
    pub fn interpret(&mut self, ) -> Result<InterpretResult, InterpretError> {
        Ok(InterpretResult)
    }

    pub fn instruct(&mut self, chunk: &'a Chunk) -> Result<(), InterpretError> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value, InterpretError> {
        self.stack.pop().ok_or(InterpretError::RuntimeError)
    }

    fn chunk(&self) -> Result<&'a Chunk, InterpretError> {
        self.chunk.ok_or(InterpretError::RuntimeError)
    }

    fn read_op(&mut self) -> Result<OpCode, InterpretError> {
        let op = self.chunk()?.read_op(self.ip)?;
        self.ip += 1;
        Ok(op)
    }

    fn read_byte(&mut self) -> Result<u8, InterpretError> {
        let op = self.chunk()?.read(self.ip)?;
        self.ip += 1;
        Ok(op)
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), InterpretError>
    where F: Fn(Value, Value) -> Value {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(a, b));
        Ok(())
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            match self.read_op()? {
                OpCode::Return => {
                    println!("{}", self.pop()?);
                    break;
                },
                OpCode::Constant => {
                    let constant = self.chunk()?
                                       .read_constant(self.read_byte()?.into())?;
                    self.push(constant);
                },
                OpCode::ConstantLong => {
                    let mut idx: usize = 0;
                    for _ in 0..=2 {
                        let b: usize = self.read_byte()?.into();
                        idx = (idx << 2) + b;
                    }

                    let constant = self.chunk()?.read_constant(idx)?;
                    self.push(constant);
                },
                OpCode::Add => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide => self.binary_op(|a, b| a / b)?,
                OpCode::Negate => {
                    let v = self.pop()?;
                    self.push(-v);
                },
            };
        }
        Ok(())
    }
}
