rust
#[allow(non_camel_case_types)]
struct i(i64);

fn ignorant_fn() {
    let i = 0; // ERROR: let bindings cannot shadow tuple structs
    println!("{}", i); 
}
