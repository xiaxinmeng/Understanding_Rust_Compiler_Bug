rust
trait Memory {
    type Provenance;
    
    fn foo();
}

struct BasicMemory;

impl Memory for BasicMemory {
    type Provenance = ();
    fn foo() {}
}

fn some_fn() {
    let y = BasicMemory::foo();
}

fn some_generic_fn<T: Memory>() {
    let y = T::foo();
}
