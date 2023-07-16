 Rust
#![feature(associated_consts)]

trait HasId {
    const ID: i32;
}

fn get_id<T: HasId + ?Sized>() -> i32 {
    <T as HasId>::ID
}

fn main() {
}
