rust
#[allow(non_camel_case_types)]
struct i(i64);

macro ignorant_macro() {
    let i = 0; // ERROR: let bindings cannot shadow tuple structs
    println!("{}", i); 
}

fn main() {
    ignorant_macro!(); // NOTE: in this macro invocation
}
