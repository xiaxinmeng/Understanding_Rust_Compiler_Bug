rust
#[cfg(first)]
struct CycleOne(Box<CycleTwo>);

#[cfg(second)]
enum CycleOne {
    Variant(Box<CycleTwo>)
}

struct CycleTwo(CycleOne);

fn assert_send<T: Send>() {}

fn bar() {
    assert_send::<CycleOne>();
    assert_send::<CycleTwo>();
}
