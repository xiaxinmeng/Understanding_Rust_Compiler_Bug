
[01:11:38] -error[E0512]: transmute called with types of different sizes
[01:11:38] +error[E0591]: can't transmute zero-sized type
[01:11:38]    --> $DIR/transmute-from-fn-item-types-error.rs:14:13
[01:11:38]     |
[01:11:38]  14 |     let i = mem::transmute(bar);
[01:11:38]     |             ^^^^^^^^^^^^^^
[01:11:38]     |
[01:11:38] -   = note: source type: unsafe fn() {bar} (0 bits)
[01:11:38] -   = note: target type: i32 (32 bits)
[01:11:38] +   = note: source type: unsafe fn() {bar}
[01:11:38] +   = note: target type: i32
[01:11:38] +   = help: cast with `as` to a pointer instead
