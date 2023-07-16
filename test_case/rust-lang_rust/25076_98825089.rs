 Rust
#![feature(no_std,lang_items)]
#![no_std]
#![crate_type="rlib"]

#[lang="sized"] pub trait Sized {}

struct S;

trait InOut<T> { type Out; }

impl MIterator<u32> for S {}

trait MIterator<T> : Sized {
    fn _fold<B, F: InOut<B, Out=B>>(self, init: B, f: F) -> B { loop {} }
}

fn bot<T>() -> T { loop {} }

fn utopian_tree() {
    <S as MIterator<u32>>::_fold(S, bot(), ());
}

fn main() {}
