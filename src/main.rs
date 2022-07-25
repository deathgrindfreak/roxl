extern crate rlox;

use rlox::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::default();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Return, 123);
    chunk.disassemble_chunk("test chunk")
}
