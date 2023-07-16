rust
static THIS: RwLock<Cell<i32>>;
THIS.read().unwrap().with(|inner| {
    THIS.read().unwrap().set(2);
})
