 rust
use self::Instruction::*;

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Increment {
        amount: i8,
    },
    Loop {
        body: Vec<Instruction>,
    },
}

fn is_zero_inc(instr: &Instruction) -> bool {
    if let Increment { amount: 0 } = *instr {
        return false;
    }
    true
}

fn combine_increments(instrs: Vec<Instruction>) -> Vec<Instruction> {
    instrs.into_iter()
          .filter(is_zero_inc)
          .collect()
}

fn main() {
    let instrs = vec![Increment { amount: 1 },
                      Loop { body: vec![Increment { amount: -1 }] },
                      Increment { amount: 1 },
                      Increment { amount: 1 }];
    println!("{:?}", combine_increments(instrs));
}
