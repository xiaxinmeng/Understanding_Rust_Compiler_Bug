
$ $ rustup run nightly rustc --emit asm --crate-type=rlib --target armv7-linux-androideabi foo.rs && fgrep vld foo.s
warning: field is never used: `state`, #[warn(dead_code)] on by default
 --> foo.rs:4:5
  |
4 |     state: State,
  |     ^^^^^^^^^^^^

	vldr	d0, [r11, #-24]
