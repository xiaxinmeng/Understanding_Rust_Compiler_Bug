rust
fn oh_no1() -> i32 {
    loop {
         return 1;
    }
    // we abort here since the fake edge means MIR typeck tries to assert that () == i32
    // even though the only way we can get here is if we're unwinding and at that point the
    // return value doesn't matter
}
