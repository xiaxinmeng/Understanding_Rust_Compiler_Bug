rust
static A: AtomicIsize = AtomicIsize::new(0);
#[test]
fn foo() {
    assert!(A.load(SeqCst) == 0);
}
