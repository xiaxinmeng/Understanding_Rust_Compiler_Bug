rust
trait Bar {}

fn foo<T: Bar>(t: T, s: String) {}

fn bar<T: Bar>(t: T, u: u32) {}

fn main() {
    foo(String::new(), String::new());
 // ^^^ for foo, the error is placed here
    
    // compare to bar
    bar(String::new(), 1u32);
    //  ^^^^^^^^^^^ where the error is pointing out the correct arg
}
