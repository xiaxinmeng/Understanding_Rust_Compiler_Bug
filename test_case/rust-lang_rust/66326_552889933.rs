rust
const BAR: &i32 = &42;
match &0 {
    BAR => {}
    _ => {}
}
