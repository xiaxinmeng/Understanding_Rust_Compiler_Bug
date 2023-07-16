rs
#![feature(allocator_api)]
#![feature(std_internals)]

use std::alloc::{Allocator, Global};

#[derive(PartialOrd, Eq, Ord)]
pub struct String<#[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    vec: Vec<u8, A>,
}
impl<A: Allocator, B: Allocator> PartialEq<String<B>> for String<A> {
    fn eq(&self, other: &String<B>) -> bool { todo!() }
}
