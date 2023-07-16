
[01:20:46] -error[E0512]: transmute called with types of different sizes
[01:20:46] +error[E0591]: can't transmute zero-sized type
[01:20:46]    --> $DIR/transmute-from-fn-item-types-error.rs:16:13
[01:20:46]     |
[01:20:46]  16 |     let i = mem::transmute(bar);
[01:20:46]     |             ^^^^^^^^^^^^^^
[01:20:46]     |
[01:20:46] -   = note: source type: unsafe fn() {bar} (0 bits)
[01:20:46] -   = note: target type: i32 (32 bits)
[01:20:46] +   = note: source type: unsafe fn() {bar}
[01:20:46] +   = note: target type: i32
[01:20:46] +   = help: cast with `as` to a pointer instead
