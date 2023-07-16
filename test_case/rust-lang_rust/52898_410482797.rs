
fn do_a_thing (_: &u8) {}

fn main () {
    unsafe {
        let x: u8 = std::mem::uninitialized();
        do_a_thing(&x);
    }
}
