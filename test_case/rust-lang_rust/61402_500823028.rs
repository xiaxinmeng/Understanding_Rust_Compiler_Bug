rust
#![allow(unused_imports, dead_code)]

pub struct U1;

pub trait ArrayLength {  }
impl ArrayLength for U1 { }

pub trait Trait { type Output; }
impl Trait for U1 { type Output = U1; }

pub trait DimName { type Value; }
impl DimName for U1 { type Value = U1; }

pub struct ArrayStorage<R>
where
    R: DimName,
    R::Value: Trait,
    <R::Value as Trait>::Output: ArrayLength,
{
    data: <R::Value as Trait>::Output,
}

pub struct DefaultAllocator;

pub trait Allocator<R = U1> { type Buffer; }
impl<R: DimName> Allocator<R> for DefaultAllocator
where
    R::Value: Trait,
    <R::Value as Trait>::Output: ArrayLength,
{
    type Buffer = ArrayStorage<R>;
}

#[repr(C, packed)]
struct Vertex {
    pos: <DefaultAllocator as Allocator>::Buffer
}

fn main() { println!("Hello, world!"); }
