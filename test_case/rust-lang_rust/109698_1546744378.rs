plain
   Compiling object v0.30.1
   Compiling hashbrown v0.13.1
   Compiling miniz_oxide v0.6.2
   Compiling addr2line v0.19.0
error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1174 |         self.bytes().eq(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1174 |         self.bytes().eq(other.bytes())
     |                               ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1201 |         self.bytes().partial_cmp(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1201 |         self.bytes().partial_cmp(other.bytes())
     |                                        ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1205 |         self.bytes().lt(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1205 |         self.bytes().lt(other.bytes())
     |                               ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1209 |         self.bytes().le(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1209 |         self.bytes().le(other.bytes())
     |                               ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1213 |         self.bytes().gt(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1213 |         self.bytes().gt(other.bytes())
     |                               ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1217 |         self.bytes().ge(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1217 |         self.bytes().ge(other.bytes())
     |                               ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1236 |         self.bytes().cmp(other.bytes())
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1236 |         self.bytes().cmp(other.bytes())
     |                                ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1286 |         self.bytes().hash(state)
     |              ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
196 |             s.bytes().len()
    |               ^^^^^ method not found in `&OsStr`
    |
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
333 |     if file.bytes() == b".." {
    |             ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
341 |     let mut iter = file.bytes().rsplitn(2, |b| *b == b'.');
    |                         ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
352 |     let slice = file.bytes();
    |                      ^^^^^ method not found in `&OsStr`
    |
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1481 |             Some(f) => f.bytes(),
     |                          ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for struct `OsString` in the current scope
     |
     |
1486 |         let start = self.inner.bytes().as_ptr().addr();
     |                                ^^^^^ method not found in `OsString`
    ::: library/std/src/ffi/os_str.rs:91:1
     |
91   | pub struct OsString {
91   | pub struct OsString {
     | ------------------- method `bytes` not found for this struct
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
     |
     |
1491 |         let new = extension.bytes();
     |                             ^^^^^ method not found in `&OsStr`
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for struct `OsStr` in the current scope
     |
2015 |         self.inner.bytes()
     |                    ^^^^^ method not found in `OsStr`
     |
     |
    ::: library/std/src/ffi/os_str.rs:119:1
     |
119  | pub struct OsStr {
     | ---------------- method `bytes` not found for this struct
     = help: items from traits can only be used if the trait is implemented and in scope
     = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
     |
568  | pub trait Read {
     | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
   --> library/std/src/sys/common/small_c_string.rs:22:36
    |
22  |     run_with_cstr(path.as_os_str().bytes(), f)
    |                                    ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
33  |     let path_os = path.as_os_str().bytes();
    |                                    ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
167 |         if program.bytes().starts_with(b"/") {
    |                    ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
169 |         } else if program.bytes().contains(&b'/') {
    |                           ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^

