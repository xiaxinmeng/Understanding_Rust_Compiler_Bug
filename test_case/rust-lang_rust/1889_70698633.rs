
fn bye() -> ! { panic!() }

fn warns() { bye(); 1 + 1; }

fn does_not_warn() { 1 + bye(); }

fn main() {}
