 rust
static w : *int = 0 as *int;
fn main() {
    let wval : int;
    io::println(fmt!("w: %u", w as uint));
    io::println(fmt!("w == null: %b", w == ptr::null()));
    unsafe { wval = *w; }
    io::println(fmt!("*w: %d", wval));
}
