use crate::value::Value;
use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::error::{InterpretError};

#[derive(Default)]
pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

pub struct InterpretResult;

impl VM {
    pub fn interpret(&mut self, source: &str) -> Result<InterpretResult, InterpretError> {
        self.chunk = Some(Chunk::default());

        if compile(source, self.chunk.as_mut().unwrap()).is_err() {
            return Err(InterpretError::CompileError);
        }

        self.ip = 0;
        self.run()
    }

    pub fn instruct(&mut self, chunk: Chunk) -> Result<InterpretResult, InterpretError> {
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

    fn peek(&mut self, distance: usize) -> Result<Value, InterpretError> {
        self.stack.get(self.stack.len() - distance - 1)
                  .cloned()
                  .ok_or(InterpretError::RuntimeError)
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn runtime_error(&mut self, msg: &'static str) {
        println!("{}", msg);

        let instruction = self.ip - self.chunk().expect("Expected chunk").code.len() - 1;
        let line = self.chunk().expect("Expected chunk").get_line(instruction);
        println!("[line {}] in script", line.expect("Expected line"));
        self.reset_stack();
    }

    fn chunk(&self) -> Result<&Chunk, InterpretError> {
        self.chunk.as_ref().ok_or(InterpretError::RuntimeError)
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
    where
        F: Fn(Value, Value) -> Result<Value, InterpretError>
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(a, b)?);
        Ok(())
    }

    fn run(&mut self) -> Result<InterpretResult, InterpretError> {
        loop {
            match self.read_op()? {
                OpCode::Return => { break; },
                OpCode::Constant => {
                    let b = self.read_byte()?.into();
                    let constant = self.chunk()?.read_constant(b)?;
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
                OpCode::Nil => self.push(Value::Nil),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Add => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide => self.binary_op(|a, b| a / b)?,
                OpCode::Not => {
                    match self.pop()? {
                        Value::Bool(b) => self.push(Value::Bool(!b)),
                        Value::Nil => self.push(Value::Bool(true)),
                        _ => return Err(InterpretError::ValueError("Expected falsable type")),
                    }
                },
                OpCode::Negate => {
                    let v = self.pop()?;
                    self.push((-v)?);
                },
            };
        }
        Ok(InterpretResult)
    }
}
