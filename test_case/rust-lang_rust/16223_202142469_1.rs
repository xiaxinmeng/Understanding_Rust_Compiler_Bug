 rust
        match *self {
            E::A(..) => self,
            e => match e {              // workaround #16223
                E::B(a, b) => Box::new(E::B(b, a)), 
                _ => unreachable!(),    // ugly :(
            }
        }
