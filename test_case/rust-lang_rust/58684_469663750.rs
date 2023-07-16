rust
use std::cell::RefCell;
use std::mem;
use std::mem::ManuallyDrop;
use std::rc::Rc;

struct SomeStruct { field: i32 }

struct ArrayDeque<A> { xs: ManuallyDrop<A> }

struct BigStruct { field: ArrayDeque<[Rc<SomeStruct>; 1024]> }

impl BigStruct {
    pub fn new() -> Self {
        unsafe {
            BigStruct {
                field: ArrayDeque {
                    xs: ManuallyDrop::new(mem::uninitialized()),
                }
            }
        }
    }

    pub fn foo(&self) -> i32 { 1 }
}

thread_local! {
    static GM: RefCell<BigStruct> = RefCell::new(BigStruct::new())
}

fn main() {
    GM.with(|gm| gm.borrow().foo() );
    println!("finish...");
}
