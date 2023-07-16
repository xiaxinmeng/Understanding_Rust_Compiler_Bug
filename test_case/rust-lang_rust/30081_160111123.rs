 rust
        self.map(|mut instr| {
            if let Instruction::Loop(ref mut body) = *instr {
                *body = f( ::std::mem::replace(body, Vec::new()) );
            }
            instr
        }).collect()

