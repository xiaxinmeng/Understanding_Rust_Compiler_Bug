 rust
let orig_handler = panic::take_handler();
panic::set_handler(|_| ());
<an actual thing>
panic::set_handler(orig_handler);
