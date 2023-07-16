 rust
#![feature(placement_in_syntax)]
#![feature(placement_new_protocol)]
use std::ops::{Placer, Place, InPlace};
use std::marker::PhantomData;

pub struct MyVec<T>(PhantomData<T>);
pub struct MyVecEmplace<T>(PhantomData<T>);

impl<T> Placer<T> for MyVec<T> {
    type Place = MyVecEmplace<T>;
    fn make_place(self) -> MyVecEmplace<T> { MyVecEmplace(PhantomData) }
}

impl<T> Place<T> for MyVecEmplace<T> {
    fn pointer(&mut self) -> *mut T { 0 as *mut _ }
}

impl<T> InPlace<T> for MyVecEmplace<T> {
    type Owner = ();
    unsafe fn finalize(self) -> () {}
}

fn main(){
    let v = MyVec(PhantomData);
    in v {42i32};
    in v {42};
}
