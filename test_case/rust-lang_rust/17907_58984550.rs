 rust
#![feature(unboxed_closures)]
#![feature(overloaded_calls)]

fn action<T, F: FnOnce(Option<T>)>(val: T, cb: F) {
    cb(Some(val));
}

pub fn main() {
    let (tx, rx) = channel::<&'static str>();

    //     Required for compiling     vv
    action("zomg", |:v: /* Option<&'static str> */| tx.send(v.unwrap()));
    assert_eq!(rx.recv(), "zomg");

    println!("Done");
}
