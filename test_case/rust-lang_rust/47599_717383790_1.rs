rust
extern "C" {
    fn db_next(iterator_id: u32) -> (u32, u32);
}
