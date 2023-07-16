rust
let x = m.lock().unwrap_or_else(|e| {
    e.get_mut().corrupted_state.reset();
    m.clear_poison();
    e.into_inner()
});
x.stuff.do_something();
// (implicit) drop(x);
