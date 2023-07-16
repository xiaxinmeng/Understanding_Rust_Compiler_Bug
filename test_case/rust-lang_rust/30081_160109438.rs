 rust
#[derive(Debug, Clone)]
pub enum Instruction {
    Increment {
        amount: ::std::num::Wrapping<i8>,
    },
    Loop(Vec<Box<Instruction>>),
}

/// Defines a method on iterators to map a function over all loop bodies.
trait MapLoopsExt: Iterator<Item=Box<Instruction>> {
    fn map_loops<F>(&mut self, f: F) -> Vec<Box<Instruction>>
        where F: Fn(Vec<Box<Instruction>>) -> Vec<Box<Instruction>>
    {
        self.map(|instr| {
            match *instr {
                Instruction::Loop(body) => Box::new( Instruction::Loop(f(body)) ),
                other => Box::new(other)
            }
        }).collect()
    }
}

impl<I> MapLoopsExt for I where I: Iterator<Item=Box<Instruction>> { }

/// Remove any loops where we know the current cell is zero.
pub fn remove_dead_loops(instrs: Vec<Box<Instruction>>) -> Vec<Box<Instruction>> {
    instrs.clone()
          .into_iter()
          .enumerate()
          .map(|(_, instr)| instr)
          .map_loops(remove_dead_loops)
}

fn main() {
    let mut instrs = vec![];
    instrs = remove_dead_loops(instrs);
    println!("{:?}", instrs);
}
