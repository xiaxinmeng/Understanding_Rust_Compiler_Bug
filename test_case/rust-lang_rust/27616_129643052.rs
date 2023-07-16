 rust
use std::cell::Cell;
use std::fmt::Debug;

fn transmute<X, Y: Debug>(a: Y, dummy: X, x: &'static mut Result<X, Y>,
                    y: &'static mut Result<X, Y>) -> &'static mut X {
    *x = Ok(dummy);
    let foo = x.as_mut().unwrap();
    *y = Err(a);
    return foo;
}

static mut FOO : Result<Option<&'static Cell<usize>>, usize> = Err(123);

fn main() {
    let baz : usize = 0xdeadbeef;

    let myfoo = unsafe { &mut FOO };

    let addr = &baz as *const usize as usize;
    let oops = transmute(addr, None, myfoo, myfoo);
    println!("Trying to print at address 0x{0:x}", addr);
    oops.map(|o| {
        println!("0x{0:x}", o.get());
        o.set(0xabcdef);
        println!("0x{0:x}", baz);
    });
}
