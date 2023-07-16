rust
trait T1 {
}

trait T2 {
}

impl<'a> T1 for &'a T2 {
}

struct S {
}

impl T2 for S {
}

fn main() {
    let t2:&T2 = &S {};
    let t1:&T1 = &t2; //This is OK
    let t3:&T1 = t2; //E0308: Expecting `T1`, found `T2`
}
