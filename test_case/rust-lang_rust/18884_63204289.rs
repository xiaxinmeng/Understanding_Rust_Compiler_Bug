 rust

fn foo(bar: *const u8) {
    let a = unsafe { (bar as *const Writer) };
}

fn main() {
}
