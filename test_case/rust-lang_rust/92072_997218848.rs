plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: use `self.print(value)` or `fmt::Trait::fmt(&value, self.out)`, instead of `write!(self.out, "{...}", value)`
    |
5   |  / macro_rules! write {
5   |  / macro_rules! write {
6   |  |     ($($ignored:tt)*) => {
    |  |_________^
    |  |_________^
8   | ||             "use `self.print(value)` or `fmt::Trait::fmt(&value, self.out)`, \
9   | ||              instead of `write!(self.out, \"{...}\", value)`"
    | ||_________^
11  |  |     };
12  |  | }
    |  |_- in this expansion of `write!`
    |  |_- in this expansion of `write!`
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/lib.rs:254:21
    |
254 |                        write!(size_limited_fmt, "{:#}", d)


error: use `self.print(value)` or `fmt::Trait::fmt(&value, self.out)`, instead of `write!(self.out, "{...}", value)`
    |
5   |  / macro_rules! write {
5   |  / macro_rules! write {
6   |  |     ($($ignored:tt)*) => {
    |  |_________^
    |  |_________^
8   | ||             "use `self.print(value)` or `fmt::Trait::fmt(&value, self.out)`, \
9   | ||              instead of `write!(self.out, \"{...}\", value)`"
    | ||_________^
11  |  |     };
12  |  | }
    |  |_- in this expansion of `write!`
    |  |_- in this expansion of `write!`
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.21/src/lib.rs:256:21
    |
256 |                        write!(size_limited_fmt, "{}", d)

error: could not compile `rustc-demangle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
