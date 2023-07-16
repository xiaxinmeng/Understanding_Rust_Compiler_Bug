rust
#![feature(allocator_api)]

fn main(){
    let vec = Vec::new_in(&std::alloc::Global);

    let _: Box<[()], _> = vec.into_boxed_slice();
}
