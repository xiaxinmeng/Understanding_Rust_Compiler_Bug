rust
fn does_not_work() -> usize {
    #[cfg(not(test))] { 0 }
    #[cfg(test)] { 1 }
}
