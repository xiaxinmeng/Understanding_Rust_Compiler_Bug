

../src/libstd\sys/windows\ext\fs.rs:24:1: 49:2 error: non-deprecated unstable items need to point to an issue with `issue = "NNN"`
../src/libstd\sys/windows\ext\fs.rs:24 pub trait OpenOptionsExt {
../src/libstd\sys/windows\ext\fs.rs:25     /// Overrides the `dwDesiredAccess` argument to the call to `CreateFile`
../src/libstd\sys/windows\ext\fs.rs:26     /// with the specified value.
../src/libstd\sys/windows\ext\fs.rs:27     fn desired_access(&mut self, access: u32) -> &mut Self;
../src/libstd\sys/windows\ext\fs.rs:28 
../src/libstd\sys/windows\ext\fs.rs:29     /// Overrides the `dwCreationDisposition` argument to the call to
                                       ...
