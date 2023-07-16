
> ~/.cargo/bin/rustup run stable rustc test.rs --test
test.rs:5:5: 5:6 error: expected one of `.`, `;`, or an operator, found `}`
test.rs:5     }}
              ^
fatal runtime error: out of memory
> ~/.cargo/bin/rustup run nightly rustc test.rs --test
test.rs:5:5: 5:6 error: expected one of `.`, `;`, `?`, or an operator, found `}`
test.rs:5     }}
              ^
fatal runtime error: out of memory
