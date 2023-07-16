rust
trait T1 { fn f(&self); }
trait T2 { fn g(&self); }

struct S;

impl T1 for S {
    fn f(&self) {
        println!("<S as T1>::f");
    }
}

impl<'a> T2 for &'a T1 {
    fn g(&self) {
        println!("<&T1 as T2>::g");
        self.f();
    }
}

fn main() {
    let s = &S;

    let t1_object: &T1 = s;
    let t2_object: &T2 = &t1_object;    
}
