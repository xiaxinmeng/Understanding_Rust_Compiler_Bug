 rust
struct Foo;

const OFFSET_OF_C: *const () = unsafe {
    &*(0 as *const Foo)as *const _ as *const _
};

fn main() {
    println!("offset of c {}" , OFFSET_OF_C as usize)
}
