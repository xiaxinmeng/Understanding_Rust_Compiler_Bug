rust
enum Void {}

fn foo(ptr: *const Void) {
    let _ = *ptr;
}

foo(&3 as *const _ as *const _);
