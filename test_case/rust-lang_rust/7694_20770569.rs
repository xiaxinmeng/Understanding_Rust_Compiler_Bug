
fn main() {
    let dangling_ptr: *u8 = CStr2::new(5).as_ref();
    std::io::println(fmt!("here: %?", dangling_ptr));
}
