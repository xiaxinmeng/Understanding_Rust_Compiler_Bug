
error[E0308]: mismatched types
    --> src/main.rs:9:10
     |
9    |     func(value);
     |     ---- ^^^^^ expected struct `log::SetLoggerError`, found a different struct `log::SetLoggerError`
     |     |
     |     arguments to this function are incorrect
     |
     = note: struct `log::SetLoggerError` and struct `log::SetLoggerError` have similar names, but are actually distinct types
note: struct `log::SetLoggerError` is defined in the current crate
    --> src/main.rs:2:5
     |
2    |     pub struct SetLoggerError;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^
note: struct `log::SetLoggerError` is defined in crate `log`
    --> /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.17/src/lib.rs:1541:1
     |
1541 | pub struct SetLoggerError(());
     | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: function defined here
    --> src/main.rs:5:4
     |
5    | fn func(_arg: ::log::SetLoggerError) {}
     |    ^^^^ ---------------------------
