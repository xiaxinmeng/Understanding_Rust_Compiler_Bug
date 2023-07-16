rust
let x = m.lock().unwrap_or_else(|e| {
    e.get_mut().corrupted_state.reset();
    e.clear_poison()
});
x.stuff.do_something();
// (implicit) drop(x);
