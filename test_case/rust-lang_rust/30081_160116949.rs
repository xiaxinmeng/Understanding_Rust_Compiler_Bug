 rust
pub enum Instruction {
    Increment(i8),
    Loop(Box<Box<()>>),
}

fn main() {
    let instrs: Option<(u8, Box<Instruction>)> = None;
    instrs.into_iter()
          .map(|(_, instr)| instr)
          .map(|instr| match *instr { _other => {} })
          .last();
}
