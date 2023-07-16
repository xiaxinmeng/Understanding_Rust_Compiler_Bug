rust
#![allow(unused)]

trait A {
    fn foo(&self) -> Self where Self: Copy;
}

impl A for [fn(&())] {
    fn foo(&self) -> Self where Self: Copy { *(&[] as &[_]) }
}

impl A for i32 {
    fn foo(&self) -> Self { 3 }
}
