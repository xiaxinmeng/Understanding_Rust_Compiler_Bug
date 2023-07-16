rust
const FOO: usize {
    static FOO: AtomicUsize = AtomicUsize::new(0);
    *(&FOO as *const _ as *const usize)
};
