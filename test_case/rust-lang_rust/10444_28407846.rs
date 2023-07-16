
struct MemoryPool { ... } // this is private and becomes an implementation detail

impl Arena {
    fn init() -> Arena { ... } // this is private!

    pub fn with<A, R>(a: A, f: <'a> |&'a mut Arena<'a>, A| -> R) -> R {
        let mut pool = MemoryPool::init();
        let mut arena = Arena::init(&mut pool);
        f(&mut arena, a)
    }
}
