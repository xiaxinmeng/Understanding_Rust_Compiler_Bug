 rust
trait T1 { }

trait T2 { }

struct S1;

impl T1 for S1 { }

impl <S: T2> T1 for S { }

fn main() {}
