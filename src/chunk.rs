pub enum OpCode {
    Constant,
    ConstantLong,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<OpCode, u8> {
        match value {
            0x00 => Ok(OpCode::Constant),
            0x01 => Ok(OpCode::ConstantLong),
            0x02 => Ok(OpCode::Return),
            _ => Err(value),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        match op {
            OpCode::Constant => 0x00,
            OpCode::ConstantLong => 0x01,
            OpCode::Return => 0x02,
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<f64>,
    lines: Vec<(u32, u32)>,
}

impl Chunk {
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
        let mut curr = None;
        for &(line, count) in &self.lines {
            offset -= count as i32;
            curr = Some(line);
            if offset <= 0 {
                return Some(line)
            }
        }
        curr
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
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
            "{} {:0<4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 4
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{} {:0<4} '{}'",
            name, constant, self.constants[constant as usize]
        );
        offset + 2
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}
