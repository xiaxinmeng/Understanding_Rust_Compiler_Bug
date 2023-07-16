rust
fn repro_ref(thing: &impl FnOnce()) {
    thing();
}
