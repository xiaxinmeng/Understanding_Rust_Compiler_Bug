rust
#[repr(C)]
struct S {
    a: u32,
    b: u32,
    c: u32,
}

fn main() {
    f(&S { a: 1, b: 2, c: g() }.a);
}

fn f(a: *const u32) {
    println!("{}", unsafe { *a.add(2) });
}

fn g() -> u32 { 3 }
