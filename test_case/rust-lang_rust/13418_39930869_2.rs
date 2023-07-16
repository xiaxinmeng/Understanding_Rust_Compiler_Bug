 rust
fn with(f: |&~str|) {}

fn arg_item(&_x: &~str) {}
    //~^ ERROR cannot move out of dereference of `&`-pointer

fn arg_closure() {
    with(|&_x| ())
    //~^ ERROR cannot move out of dereference of `&`-pointer
}

fn let_pat() {
    let &_x = &~"hi";
    //~^ ERROR cannot move out of dereference of `&`-pointer
}

// =================================================================

pub fn blah2() {
    use std::rc::Rc;
    let _x = *Rc::new(~"hi");
    //~^ ERROR cannot move out of dereference of `&`-pointer
}

// =================================================================

struct S {f:~str}
impl Drop for S {
    fn drop(&mut self) { println!("{}", self.f); }
}

fn move_in_match() {
    match S {f:~"foo"} {
        S {f:_s} => {}
        //~^ ERROR cannot move out of type `S`, which defines the `Drop` trait
    }
}

fn move_in_let() {
    let S {f:_s} = S {f:~"foo"};
    //~^ ERROR cannot move out of type `S`, which defines the `Drop` trait
}

fn move_in_fn_arg(S {f:_s}: S) {
    //~^ ERROR cannot move out of type `S`, which defines the `Drop` trait
}

// =================================================================

struct A {
    a: ~int
}

fn free<T>(_: T) {}

fn blah3() {
    let a = &A { a: ~1 };
    match a.a {
        n => { free(n) }
    }
    free(a)
}

// =================================================================

fn blah4() {
    let bar = ~3;
    let _g = || {
        let _h: proc() -> int = proc() *bar; //~ ERROR cannot move out of captured outer variable
    };
}

// =================================================================

fn main() {}
