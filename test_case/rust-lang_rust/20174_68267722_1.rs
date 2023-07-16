 rust
#![feature(unboxed_closures)]
struct GradFn<F: Fn() -> uint> {f: F} // just named the element

fn main() {
    let g = GradFn{f: |&:| 2u };
    let GradFn { f } = g; // destructuring instead of accessing the element by name
                                   // to keep code as similar as possible as the previous one 
    println!("{}", f.call(())); // compiles and prints 2   
}
