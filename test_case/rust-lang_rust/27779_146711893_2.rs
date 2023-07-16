 rust
#![feature(placement_in_syntax, placement_new_protocol)]

fn main() {
    let x: Result<_, _> = MyBox::place().map(|p| in p { 1 }); // ....map(|p| p <- 1)
    // with `try { ... ? ... }` this could also be:
    // let x = try { in MyBox::place()? { 1 } }; // ... try { MyBoxPlace()? <- 1 };
    println!("{}", *x.unwrap());

    println!("{}", *func(2).unwrap());
}

fn func(val: i32) -> Result<MyBox<i32>, BadAlloc> {
    let mut x = in try!(MyBox::place()) { val }; // ... try!(MyBox::place()) <- val
    *x += 10;
    Ok(x)
}

// implementation of the `in`  procotol:

pub struct MyBoxPlace<T> {
    ptr: *mut T
}
#[derive(Debug)]
pub struct BadAlloc;

impl<T> MyBox<T> {
    pub fn place() -> Result<MyBoxPlace<T>, BadAlloc> {
        let p = unsafe {malloc(mem::size_of::<T>())};
        if p.is_null() {
            Err(BadAlloc)
        } else {
            Ok(MyBoxPlace { ptr: p as *mut T })
        }
    }
}
impl<T> ops::Placer<T> for MyBoxPlace<T> {
    type Place = Self;
    fn make_place(self) -> Self { self }
}
impl<T> ops::Place<T> for MyBoxPlace<T> {
    fn pointer(&mut self) -> *mut T { self.ptr }
}

impl<T> ops::InPlace<T> for MyBoxPlace<T> {
    type Owner = MyBox<T>;
    unsafe fn finalize(self) -> MyBox<T> {
        let p = self.ptr as *const T;
        mem::forget(self);
        MyBox { ptr: p }   
    }
}
impl<T> Drop for MyBoxPlace<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr as *mut u8);
        }
    }
}


// implementation of the pointer itself

use std::{mem, ops, ptr};

extern {
    fn malloc(x: usize) -> *mut u8;
    fn free(p: *mut u8);
}

/// Custom `Box`
pub struct MyBox<T> {
    ptr: *const T
}

// make `MyBox` behave like a pointer
impl<T> ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {&*self.ptr}    
    }
}
impl<T> ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *(self.ptr as *mut T)}
    }
}
// etc.

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop(ptr::read(self.ptr));
            free(self.ptr as *mut u8);
        }
    }
}
