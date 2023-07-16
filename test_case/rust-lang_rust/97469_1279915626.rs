rust
#![feature(let_chains)]

fn temp(b1 : bool, b2 : bool, o1 : Option<usize>, o2 : Option<usize>) -> Option<usize> {
    let res = if let Some(u1) = o1 
                  && let Some(u2) = o2 
                  && b1 = b2
                {
                    Some(u1 + u2)
                } else {
                    None
                };
                    
    res
}
