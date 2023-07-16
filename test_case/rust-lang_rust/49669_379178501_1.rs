rust
static S_FALSE: AtomicBool = AtomicBool::new(false);
static S_TRUE: AtomicBool = AtomicBool::new(true);
static S_INT: AtomicIsize  = AtomicIsize::new(0);
static S_UINT: AtomicUsize = AtomicUsize::new(0);

#[test]
fn static_init() {
    assert!(!S_FALSE.load(SeqCst));
    assert!(S_TRUE.load(SeqCst));
    assert!(S_INT.load(SeqCst) == 0);
    assert!(S_UINT.load(SeqCst) == 0);
}
