 rust
    (x, y) = (1, 2); // you cannot do this (at least not today; there may be a postponed RFC asking for it)
    let (x, y) = (1, 2); // but you can do this
    
    let x.foo = 3; // you cannot do this (and almost certainly never will be able to)
    x.foo = 3; // but you can do this
    