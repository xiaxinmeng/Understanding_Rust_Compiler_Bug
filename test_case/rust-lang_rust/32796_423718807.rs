rust
fn foo(_x: &[u32]) { }

fn main() {
    foo(&[1, #[cfg(not(now))] 2, 3])
}
