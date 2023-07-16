
fn foo(_: u8) -> u8 { 0 }
fn bar(_: u8) -> u8 { 1 }
fn types_eq<T>(_: T, _: T) {}
    
types_eq(foo, bar);
