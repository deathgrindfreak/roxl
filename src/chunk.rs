use crate::value::Value;
use crate::error::ChunkError;

pub enum OpCode {
    Constant,
    ConstantLong,
    Nil,
    True,
    False,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = ChunkError;

    fn try_from(value: u8) -> Result<OpCode, ChunkError> {
        match value {
            0x00 => Ok(OpCode::Constant),
            0x01 => Ok(OpCode::ConstantLong),
            0x02 => Ok(OpCode::Nil),
            0x03 => Ok(OpCode::True),
            0x04 => Ok(OpCode::False),
            0x05 => Ok(OpCode::Add),
            0x06 => Ok(OpCode::Subtract),
            0x07 => Ok(OpCode::Multiply),
            0x08 => Ok(OpCode::Divide),
            0x09 => Ok(OpCode::Not),
            0x0A => Ok(OpCode::Negate),
            0x0B => Ok(OpCode::Return),
            _ => Err(ChunkError::BadOPCodeError(value)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        match op {
            OpCode::Constant => 0x00,
            OpCode::ConstantLong => 0x01,
            OpCode::Nil => 0x02,
            OpCode::True => 0x03,
            OpCode::False => 0x04,
            OpCode::Add => 0x05,
            OpCode::Subtract => 0x06,
            OpCode::Multiply => 0x07,
            OpCode::Divide => 0x08,
            OpCode::Not => 0x09,
            OpCode::Negate => 0x0A,
            OpCode::Return => 0x0B,
        }
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<(u32, u32)>,
}

impl Chunk {
    pub fn read(&self, ip: usize) -> Result<u8, ChunkError> {
        self.code.get(ip).ok_or(ChunkError::IPOutOfBoundsError).map(|&op| op)
    }

    pub fn read_op(&self, ip: usize) -> Result<OpCode, ChunkError> {
        self.read(ip)?.try_into()
    }

    pub fn read_constant(&self, ip: usize) -> Result<Value, ChunkError> {
        self.constants.get(ip).ok_or(ChunkError::IPOutOfBoundsError).map(|&op| op)
    }

    pub fn write<U: Into<u8>>(&mut self, op: U, line: u32) {
        self.code.push(op.into());

        match self.lines.pop() {
            Some((top_line, count)) => {
                if line == top_line {
                    self.lines.push((line, count + 1));
                } else {
                    self.lines.push((top_line, count));
                    self.lines.push((line, 1));
                }
            },
            None => self.lines.push((line, 1)),
        }
    }

    pub fn get_line(&self, idx: usize) -> Option<u32>{
        let mut offset: i32 = idx as i32;
        for &(line, count) in &self.lines {
            offset -= count as i32;
            if offset < 0 {
                return Some(line)
            }
        }
        None
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    // Debug functions

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);

        let current_line = self.get_line(offset).expect("Could not find line number");
        if offset > 0 && current_line == self.get_line(offset - 1).unwrap() {
            print!("   | ");
        } else {
            print!("{:>4} ", current_line);
        }

        let op = self.code[offset];
        match op.try_into() {
            Ok(OpCode::Constant) => self.constant_instruction("OP_CONSTANT", offset),
            Ok(OpCode::ConstantLong) => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            Ok(OpCode::Nil) => Self::simple_instruction("OP_NIL", offset),
            Ok(OpCode::True) => Self::simple_instruction("OP_TRUE", offset),
            Ok(OpCode::False) => Self::simple_instruction("OP_FALSE", offset),
            Ok(OpCode::Add) => Self::simple_instruction("OP_ADD", offset),
            Ok(OpCode::Subtract) => Self::simple_instruction("OP_SUBTRACT", offset),
            Ok(OpCode::Multiply) => Self::simple_instruction("OP_MULTIPLY", offset),
            Ok(OpCode::Divide) => Self::simple_instruction("OP_DIVIDE", offset),
            Ok(OpCode::Not) => Self::simple_instruction("OP_NOT", offset),
            Ok(OpCode::Negate) => Self::simple_instruction("OP_NEGATE", offset),
            Ok(OpCode::Return) => Self::simple_instruction("OP_RETURN", offset),
            Err(_) => {
                println!("Unknown opcode: {}", op);
                offset + 1
            }
        }
    }

    fn constant_long_instruction(&self, name: &str, offset: usize) -> usize {
        let mut constant = 0;
        for o in 1..=3 {
            constant += (constant << 2) + self.code[offset + o];
        }
        println!(
            "{} {:0<4} '{:?}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 4
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{} {:0<4} '{:?}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_rle() {
        let mut chunk = Chunk::default();

        assert_eq!(chunk.get_line(0), None);
        assert_eq!(chunk.get_line(10), None);

        chunk.write(OpCode::Return, 1);
        chunk.write(OpCode::Return, 1);
        chunk.write(OpCode::Return, 1);

        for offset in 0..=2 {
            assert_eq!(chunk.get_line(offset), Some(1));
        }

        assert_eq!(chunk.get_line(10), None);

        chunk.write(OpCode::Return, 2);
        chunk.write(OpCode::Return, 2);
        chunk.write(OpCode::Return, 2);
        chunk.write(OpCode::Return, 2);

        for offset in 3..=6 {
            assert_eq!(chunk.get_line(offset), Some(2));
        }

        assert_eq!(chunk.get_line(1000), None);

        chunk.write(OpCode::Return, 3);
        chunk.write(OpCode::Return, 3);
        chunk.write(OpCode::Return, 3);
        chunk.write(OpCode::Return, 3);
        chunk.write(OpCode::Return, 3);

        for offset in 7..=11 {
            assert_eq!(chunk.get_line(offset), Some(3));
        }

        assert_eq!(chunk.get_line(10000), None);

        chunk.write(OpCode::Return, 100);
        chunk.write(OpCode::Return, 100);

        for offset in 12..=13 {
            assert_eq!(chunk.get_line(offset), Some(100));
        }
    }
}
