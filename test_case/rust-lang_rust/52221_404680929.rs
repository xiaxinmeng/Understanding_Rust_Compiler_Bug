
[01:06:30] error: trait objects without an explicit `dyn` are deprecated
[01:06:30]   --> libstd/sys/redox/process.rs:54:23
[01:06:30]    |
[01:06:30] 54 |     closures: Vec<Box<FnMut() -> io::Result<()> + Send + Sync>>,
[01:06:30]    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
[01:06:30]    |
[01:06:30] note: lint level defined here
[01:06:30]   --> libstd/lib.rs:224:9
[01:06:30]    |
[01:06:30] 224| #![deny(bare_trait_objects)]
[01:06:30]    |         ^^^^^^^^^^^^^^^^^^
[01:06:30]
[01:06:30] error: trait objects without an explicit `dyn` are deprecated
[01:06:30]    --> libstd/sys/redox/process.rs:125:31
[01:06:30]     |
[01:06:30] 125 |                        f: Box<FnMut() -> io::Result<()> + Send + Sync>) {
[01:06:30]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut() -> io::Result<()> + Send + Sync`
