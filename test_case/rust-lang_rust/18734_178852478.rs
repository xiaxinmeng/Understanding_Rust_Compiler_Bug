 rust
/// Instruction description
struct Instruction {
    /// Number of CPU clock cycles taken by the instruction to execute
    cycles:  u32,
    /// Instruction implementation
    execute: fn (),
}

impl Instruction {
    fn new(cycles: u32, execute: fn ()) -> Instruction {
        Instruction { cycles: cycles, execute: execute }
    }
}

pub static OPCODES: [Instruction; 1] = [
    Instruction::new(4, nop),
];

/// No operation
fn nop() {
}

fn main() {
}
