rust
self.try_for_each(move |x| try { 
    if predicate(&x) { return LoopState::Break(x) } 
}).break_value() 
