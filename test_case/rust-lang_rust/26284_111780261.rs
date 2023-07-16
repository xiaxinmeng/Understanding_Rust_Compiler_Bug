 rust
let my_fn = if debug_enabled {
    my_fn_debug
} else {
    my_fn_fast
};
my_fn(...);
