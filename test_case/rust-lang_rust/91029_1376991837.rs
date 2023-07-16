rust
fn main() {
    let mut data = false;
    let ptr = &mut data as *mut bool;
    let safe_ref = unsafe {&* ptr };
    match safe_ref {
        &false if {
            unsafe { *ptr = true };
            false
        } => unreachable!(),
        &false => println!("A"),
        &true => println!("B"),
    }
}
