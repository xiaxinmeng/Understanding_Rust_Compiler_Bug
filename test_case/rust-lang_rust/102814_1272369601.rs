plain
[RUSTC-TIMING] gimli test:false 4.166
[RUSTC-TIMING] object test:false 4.599
warning: dropping unsupported crate type `dylib` for target `wasm32-wasi`

error[E0277]: the trait bound `io::stdio::Stdin: owned::AsFd` is not satisfied
     |
1034 | / macro_rules! impl_is_terminal {
1034 | / macro_rules! impl_is_terminal {
1035 | |     ($($t:ty),*$(,)?) => {$(
1036 | |         #[unstable(feature = "sealed", issue = "none")]
1037 | |         impl crate::sealed::Sealed for $t {}
1043 | |                 crate::sys::io::is_terminal(self)
1043 | |                 crate::sys::io::is_terminal(self)
     | |                 --------------------------- ^^^^ the trait `owned::AsFd` is not implemented for `io::stdio::Stdin`
     | |                 required by a bound introduced by this call
...    |
1046 | |     )*}
1047 | | }
1047 | | }
     | |_- in this expansion of `impl_is_terminal!`
1048 |
1049 |   impl_is_terminal!(File, Stdin, StdinLock<'_>, Stdout, StdoutLock<'_>, Stderr, StderrLock<'_>);
     |
     |
     = help: the following other types implement trait `owned::AsFd`:
               &T
               Arc<T>
               Box<T>
               Socket
               StderrLock<'a>
               StderrLock<'a>
               StdinLock<'a>
               StdoutLock<'a>
             and 11 others
note: required by a bound in `is_terminal`
    --> library/std/src/sys/wasi/io.rs:76:30
     |
76   | pub fn is_terminal(fd: &impl AsFd) -> bool {
     |                              ^^^^ required by this bound in `is_terminal`

error[E0277]: the trait bound `io::stdio::Stdout: owned::AsFd` is not satisfied
     |
1034 | / macro_rules! impl_is_terminal {
1034 | / macro_rules! impl_is_terminal {
1035 | |     ($($t:ty),*$(,)?) => {$(
1036 | |         #[unstable(feature = "sealed", issue = "none")]
1037 | |         impl crate::sealed::Sealed for $t {}
1043 | |                 crate::sys::io::is_terminal(self)
1043 | |                 crate::sys::io::is_terminal(self)
     | |                 --------------------------- ^^^^ the trait `owned::AsFd` is not implemented for `io::stdio::Stdout`
     | |                 required by a bound introduced by this call
...    |
1046 | |     )*}
1047 | | }
1047 | | }
     | |_- in this expansion of `impl_is_terminal!`
1048 |
1049 |   impl_is_terminal!(File, Stdin, StdinLock<'_>, Stdout, StdoutLock<'_>, Stderr, StderrLock<'_>);
     |
     |
     = help: the following other types implement trait `owned::AsFd`:
               &T
               Arc<T>
               Box<T>
               Socket
               StderrLock<'a>
               StderrLock<'a>
               StdinLock<'a>
               StdoutLock<'a>
             and 11 others
note: required by a bound in `is_terminal`
    --> library/std/src/sys/wasi/io.rs:76:30
     |
76   | pub fn is_terminal(fd: &impl AsFd) -> bool {
     |                              ^^^^ required by this bound in `is_terminal`

error[E0277]: the trait bound `io::stdio::Stderr: owned::AsFd` is not satisfied
     |
1034 | / macro_rules! impl_is_terminal {
1034 | / macro_rules! impl_is_terminal {
1035 | |     ($($t:ty),*$(,)?) => {$(
1036 | |         #[unstable(feature = "sealed", issue = "none")]
1037 | |         impl crate::sealed::Sealed for $t {}
1043 | |                 crate::sys::io::is_terminal(self)
1043 | |                 crate::sys::io::is_terminal(self)
     | |                 --------------------------- ^^^^ the trait `owned::AsFd` is not implemented for `io::stdio::Stderr`
     | |                 required by a bound introduced by this call
...    |
1046 | |     )*}
1047 | | }
1047 | | }
     | |_- in this expansion of `impl_is_terminal!`
1048 |
1049 |   impl_is_terminal!(File, Stdin, StdinLock<'_>, Stdout, StdoutLock<'_>, Stderr, StderrLock<'_>);
     |
     |
     = help: the following other types implement trait `owned::AsFd`:
               &T
               Arc<T>
               Box<T>
               Socket
               StderrLock<'a>
               StderrLock<'a>
               StdinLock<'a>
               StdoutLock<'a>
             and 11 others
note: required by a bound in `is_terminal`
    --> library/std/src/sys/wasi/io.rs:76:30
     |
76   | pub fn is_terminal(fd: &impl AsFd) -> bool {
     |                              ^^^^ required by this bound in `is_terminal`
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] std test:false 1.831
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 3 previous errors; 1 warning emitted
