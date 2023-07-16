
$ cargo +629d891499bca79aeb8ea079f756c566fdabbd3e build
   Compiling clippy_lints v0.0.212 (file:///home/manishearth/mozilla/Wall/rust-clippy/clippy_lints)                       
error: cannot find macro `__lazy_static_internal!` in this scope==>    ] 59/63: clippy_lints                              
  --> clippy_lints/src/utils/conf.rs:73:1
   |
73 | / lazy_static! {
74 | |     static ref ERRORS: Mutex<Vec<Error>> = Mutex::new(Vec::new());
75 | | }
   | |_^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

