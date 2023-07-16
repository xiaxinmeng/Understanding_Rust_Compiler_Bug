 rust
struct S;

impl S {
    fn f<B: std::fmt::Show>(&self, g: |&i32| -> B) {
        let h = |x| { g(&x) };
        let r = h(10);
        /*
        let r = g(&10_i32);
        */
        println!("r == {}", r); //  r == 32767 ???
    }
}

fn main() {
    let s = S;
    s.f(|p| {
        println!("p == {}", p);
        p
    })
}
