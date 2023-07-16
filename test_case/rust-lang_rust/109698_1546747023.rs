plain
    Checking addr2line v0.19.0
error[E0424]: expected value, found module `self`
   --> library/std/src/sys/windows/os_str.rs:161:9
    |
160 |     pub unsafe fn from_os_str_bytes_unchecked(s: &[u8]) -> &Slice {
    |                   --------------------------- this function doesn't have a `self` parameter
161 |         self.inner.from_bytes_unchecked(s)
    |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
    |
help: add a `self` receiver parameter to make the associated `fn` a method
    |
160 |     pub unsafe fn from_os_str_bytes_unchecked(&self, s: &[u8]) -> &Slice {

error[E0599]: no method named `bytes` found for reference `&OsString` in the current scope
   --> library/std/src/sys/windows/args.rs:229:25
    |
    |
229 |     let arg_bytes = arg.bytes();
    |                         ^^^^^ method not found in `&OsString`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsString` in the current scope
   --> library/std/src/sys/windows/args.rs:300:55
    |
300 |             Arg::Regular(arg) if !force_quotes => arg.bytes().iter().any(|c| SPECIAL.contains(c)),
    |                                                       ^^^^^ method not found in `&OsString`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
36  |     !path.bytes().iter().copied().any(is_sep_byte)
    |           ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
39  |     let is_verbatim = path.bytes().starts_with(br"\\?\");
    |                            ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
41  |     if let Some(&c) = path.bytes().last() { is_separator(c) } else { false }
    |                            ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
63  |         for (i, &ch) in path.bytes().iter().take(LEN).enumerate() {
    |                              ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&'a OsStr` in the current scope
    |
    |
96  |         &self.path.bytes()[..self.index]
    |                    ^^^^^ method not found in `&'a OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&'a OsStr` in the current scope
    |
    |
104 |         unsafe { bytes_as_os_str(&self.path.bytes()[self.index..]) }
    |                                             ^^^^^ method not found in `&'a OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
176 |     match path.bytes() {
    |                ^^^^^ method not found in `&OsStr`
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
185 |     if path.bytes().get(2).map(|&x| is_sep_byte(x)).unwrap_or(true) {
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
199 |     match path.bytes().iter().position(|&x| separator(x)) {
    |                ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
203 |             let component = &path.bytes()[..separator_start];
    |                                   ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
207 |             let path = &path.bytes()[separator_end..];
    |                              ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&OsStr` in the current scope
    |
    |
332 |         if path.bytes().contains(&0) {
    |                 ^^^^^ method not found in `&OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&'a OsStr` in the current scope
    |
    |
398 |         exe_path.bytes()[exe_path.len() - EXE_SUFFIX.len()..]
    |                  ^^^^^ method not found in `&'a OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `bytes` found for reference `&'a OsStr` in the current scope
    |
    |
428 |         let has_extension = exe_path.bytes().contains(&b'.');
    |                                      ^^^^^ method not found in `&'a OsStr`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `bytes`, perhaps you need to implement it
    |
568 | pub trait Read {
    | ^^^^^^^^^^^^^^

