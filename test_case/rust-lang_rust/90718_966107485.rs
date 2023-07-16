rust
fn main() {
    let mut x: Vec<u32>;
    unsafe {
        std::ptr::write(std::ptr::addr_of_mut!(x), vec![0]);
    }
    x = vec[1];
}
