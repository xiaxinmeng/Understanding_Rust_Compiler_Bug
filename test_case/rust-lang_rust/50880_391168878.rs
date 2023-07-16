rust
#![feature(allocator_api)]

fn main() {
    std::alloc::set_hook(Box::new(|layout| { panic!("oom {}", layout.size()) }));
    let result = std::panic::catch_unwind(|| {
        let v = Vec::<u8>::with_capacity(100000000000000);
        println!("{:p}", &v[..]);
    });
    println!("{:?}", result);
}
