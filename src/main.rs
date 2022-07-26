extern crate rlox;

use rlox::chunk::{Chunk, OpCode};
use rlox::vm::VM;

fn main() {
    let mut vm = VM::default();
    let mut chunk = Chunk::default();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant, 1);
    chunk.write(constant as u8, 1);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::Constant, 1);
    chunk.write(constant as u8, 1);

    chunk.write(OpCode::Add, 1);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::Constant, 1);
    chunk.write(constant as u8, 1);

    chunk.write(OpCode::Divide, 1);
    chunk.write(OpCode::Negate, 1);

    chunk.write(OpCode::Return, 1);
    chunk.disassemble_chunk("test chunk");

    vm.instruct(&chunk).unwrap();
}
