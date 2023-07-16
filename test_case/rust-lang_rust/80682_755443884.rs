rust
fn main() {
    let mut x = [String::from("1"), String::from("2"), String::from("3")];
    let x_ptr = x.as_mut_ptr();
    unsafe {
        x_ptr
            .cast::<[String; 2]>()
            .swap(x_ptr.add(1).cast::<[String; 2]>());
    }
}
