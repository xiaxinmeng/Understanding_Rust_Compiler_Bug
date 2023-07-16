rust
// Replacing filters with &[u8], &[u8; 7] or [u8; 8], things will work properly.
fn set_vertical_filter(filters: [u8; 7]) {
    // The values are correct (2, 4, 6, 8, a, c, e).
    for f in filters.iter() {
        println!("{f:x}");
    }
    // The values start four bytes earlier than they should (0, 0, 0, 7, 2, 4, 6).
    for f in filters {
        println!("{f:x}");
    }
}

fn main() {
    set_vertical_filter([2, 4, 6, 8, 10, 12, 14]);
}
