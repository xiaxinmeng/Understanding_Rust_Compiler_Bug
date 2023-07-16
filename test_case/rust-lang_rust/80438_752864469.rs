rust
#[derive(Copy, Clone, Debug)]
struct S;

    let boxed_val = Box::new(S);  
    let val = *boxed_val;
    // this copies the value out (with Deref), but the memory is not deallocated.
    println!("{:?}", boxed_val); //it's still alive now, needs another `drop` to deallocate.
    // do something more.
