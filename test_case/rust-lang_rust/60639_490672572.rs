rust
fn main() {
    let x_ptr = &[1, 2, 4] as *const [_];
    
    unsafe {
        let x = &*x_ptr;
        println!("{:?}", x.get(1).map(|x| x as *const _));
        println!("{:?}", x.get_unchecked(1) as *const _);
    }
}
