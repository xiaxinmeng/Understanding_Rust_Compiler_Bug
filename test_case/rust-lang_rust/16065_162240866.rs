 Rust
fn main() {
    // here, `s` is a stack-allocated Unicode character slice (that is, &str)
    let s = "hello, world";

    match s {
        "something" => { println!("mumbles something about types"); }
        _ => { println!("{}", s); }
    }


    // here, `t` is a heap-allocated String instance and `to_owned` is one way to get one from &str.
    let t : String = "foobar".to_owned();
    // let t = String::from("foobar"); // also works

    // the value of a String instance, *t, is an array of Unicode characters (that is `str`).  The address, `&*t`, is just &str.
    match &*t {
        "something" => { println!("mumbles something about types"); }
        _ => { println!("{}", t); }
    }    
}
