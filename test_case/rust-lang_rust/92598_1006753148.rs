rust
let prev = panic::take_hook();
panic!("in between");
panic::set_hook(new_hook);
