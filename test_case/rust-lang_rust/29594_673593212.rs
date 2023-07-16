
#![feature(thread_local)]

use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Foo {
    #[thread_local]
    bar: &'static str,
}

fn main() {
    let foo = Arc::new(Mutex::new(Foo {
        bar: "bar",  
    }));
    
    dbg!(foo.lock().unwrap().bar);
    
    let foo2 = foo.clone();
    std::thread::spawn(move || {
        foo2.lock().unwrap().bar = "baz";
    }).join().unwrap();

    dbg!(foo.lock().unwrap().bar);
}
