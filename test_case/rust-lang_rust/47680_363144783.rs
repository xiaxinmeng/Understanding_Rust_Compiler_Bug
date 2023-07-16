rust
#![feature(nll)]

fn create<'a>() -> &'a mut u32 { panic!() }
fn condition() -> bool { panic!() }
fn touch(_: &mut u32) { }

fn foo() {
    // Should be error because:
    // - if we pass through loop one time, then `q` points to `a`
    let mut a = 22;
    let mut x = &mut a;
    let mut q = create();
    
    loop {
        let p = &mut *x;
        if condition() { break; }
        q = p;
        x = create();
    }

    touch(&mut a);
    touch(q);
}

fn main() {
}
