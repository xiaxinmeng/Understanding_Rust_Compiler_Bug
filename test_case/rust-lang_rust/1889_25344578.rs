 rust
fn bye() -> ! { fail!() }

fn warns() { bye(); 1 + 1; }

fn does_not_warn() { 1 + bye(); }
