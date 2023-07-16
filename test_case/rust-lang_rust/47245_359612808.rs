rust
struct Entry {
    /* fields */
}

const NUM_ENTRIES: usize = 32;

lazy_static! {
    static ref TABLE: Box<[Entry; NUM_ENTRIES]> = {
        let mut table = Vec::<Entry>::with_capacity(NUM_ENTRIES);
        for i in 0..NUM_ENTRIES {
            table.push(Entry { /* ... */ });
        }
        table.into_boxed_slice().try_into().unwrap()
    };
}
