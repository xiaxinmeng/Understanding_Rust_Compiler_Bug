rust
    self.try_for_each(move |x| { 
        if predicate(&x) { Err(x) } 
        else { Ok(()) } 
    }).err() 
