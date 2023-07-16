plain
    Checking object v0.29.0
    Checking miniz_oxide v0.5.3
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.17.0
error[E0599]: no method named `read_buf` found for struct `AnonPipe` in the current scope
    |
358 |         self.inner.read_buf(cursor)
    |                    ^^^^^^^^ help: there is a method with a similar name: `read`
    |
    |
   ::: library/std/src/sys/windows/pipe.rs:21:1
    |
21  | pub struct AnonPipe {
    | ------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf` found for struct `AnonPipe` in the current scope
    |
427 |         self.inner.read_buf(cursor)
    |                    ^^^^^^^^ help: there is a method with a similar name: `read`
    |
    |
   ::: library/std/src/sys/windows/pipe.rs:21:1
    |
21  | pub struct AnonPipe {
    | ------------------- method `read_buf` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf`, perhaps you need to implement it
    |
554 | pub trait Read {
    | ^^^^^^^^^^^^^^

