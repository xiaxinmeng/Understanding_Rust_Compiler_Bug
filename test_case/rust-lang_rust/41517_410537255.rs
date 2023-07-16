rust
struct S;
trait T1 {}
trait T2 = T1;
impl T2 for S {}
