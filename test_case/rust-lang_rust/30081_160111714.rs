 rust
pub enum Instruction {
    Increment(i8),
    Loop(Vec<Box<Instruction>>),
}

/// Defines a method on iterators to map a function over all loop bodies.
fn map_loops<F, I>(it: I, f: F) -> Vec<Box<Instruction>>
    where F: Fn(Vec<Box<Instruction>>) -> Vec<Box<Instruction>>,
          I: Iterator<Item=Box<Instruction>>
{
    it.map(|instr| {
        match *instr {
            Instruction::Loop(body) => Box::new( Instruction::Loop(f(body)) ),
            other => Box::new(other)
        }
    }).collect()
}

/// Remove any loops where we know the current cell is zero.
pub fn remove_dead_loops(instrs: Vec<Box<Instruction>>) -> Vec<Box<Instruction>> {
    map_loops(instrs.into_iter()
          .enumerate()
          .map(|(_, instr)| instr), remove_dead_loops)
}

fn main() {
    let mut instrs = vec![];
    instrs = remove_dead_loops(instrs);
    println!("{:?}", instrs.len());
}
