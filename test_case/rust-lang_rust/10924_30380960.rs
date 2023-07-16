
use std::kinds::Pod;

// Test that type parameters with the Pod bound are implicitly copyable.

fn can_copy_POD<T:Pod>(v: T) {
    let _a = v;
    let _b = v;
}

fn main() { }
