plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error: `core::option::Option::<T>::unwrap` is not yet stable as a const fn
  --> library/std/src/sys/windows/args.rs:60:31
   |
60 |     const QUOTE: NonZeroU16 = NonZeroU16::new(b'"' as u16).unwrap();
   |
   = help: add `#![feature(const_option)]` to the crate attributes to enable

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> library/std/src/sys/windows/args.rs:80:13
   |
80 |             QUOTE => in_quotes = !in_quotes,


error: `core::option::Option::<T>::unwrap` is not yet stable as a const fn
  --> library/std/src/sys/windows/args.rs:62:31
   |
62 |     const SPACE: NonZeroU16 = NonZeroU16::new(b' ' as u16).unwrap();
   |
   = help: add `#![feature(const_option)]` to the crate attributes to enable

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> library/std/src/sys/windows/args.rs:82:13
   |
82 |             SPACE | TAB if !in_quotes => break,


error: `core::option::Option::<T>::unwrap` is not yet stable as a const fn
  --> library/std/src/sys/windows/args.rs:61:29
   |
61 |     const TAB: NonZeroU16 = NonZeroU16::new(b'\t' as u16).unwrap();
   |
   = help: add `#![feature(const_option)]` to the crate attributes to enable

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> library/std/src/sys/windows/args.rs:82:21
   |
82 |             SPACE | TAB if !in_quotes => break,

error: unreachable pattern
  --> library/std/src/sys/windows/args.rs:82:13
   |
   |
80 |             QUOTE => in_quotes = !in_quotes,
   |             ----- matches any value
81 |             // If not `in_quotes` then whitespace ends argv[0].
82 |             SPACE | TAB if !in_quotes => break,
   |             ^^^^^^^^^^^ unreachable pattern
   |
   = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
  --> library/std/src/sys/windows/args.rs:84:13
   |
   |
80 |             QUOTE => in_quotes = !in_quotes,
   |             ----- matches any value
...
84 |             _ => cur.push(w.get()),
   |             ^ unreachable pattern
error: could not evaluate constant pattern
   --> library/std/src/sys/windows/args.rs:134:22
    |
    |
134 |                 Some(QUOTE) => {

error: unreachable pattern
   --> library/std/src/sys/windows/args.rs:139:17
    |
    |
139 |                 Some(_) => in_quotes = false,

error: could not evaluate constant pattern
   --> library/std/src/sys/windows/args.rs:108:13
    |
    |
108 |             SPACE | TAB if !in_quotes => {

error: could not evaluate constant pattern
   --> library/std/src/sys/windows/args.rs:108:21
    |
    |
108 |             SPACE | TAB if !in_quotes => {


error: `core::option::Option::<T>::unwrap` is not yet stable as a const fn
  --> library/std/src/sys/windows/args.rs:59:35
   |
59 |     const BACKSLASH: NonZeroU16 = NonZeroU16::new(b'\\' as u16).unwrap();
   |
   = help: add `#![feature(const_option)]` to the crate attributes to enable

error: could not evaluate constant pattern
---

error: could not evaluate constant pattern
   --> library/std/src/sys/windows/args.rs:132:13
    |
132 |             QUOTE if in_quotes => match code_units.peek() {

error: could not evaluate constant pattern
   --> library/std/src/sys/windows/args.rs:145:13
    |
    |
145 |             QUOTE => in_quotes = true,

error: unreachable pattern
   --> library/std/src/sys/windows/args.rs:132:13
    |
    |
116 |             BACKSLASH => {
    |             --------- matches any value
...
132 |             QUOTE if in_quotes => match code_units.peek() {
    |             ^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/sys/windows/args.rs:145:13
    |
116 |             BACKSLASH => {
116 |             BACKSLASH => {
    |             --------- matches any value
...
145 |             QUOTE => in_quotes = true,
    |             ^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/sys/windows/args.rs:147:13
    |
116 |             BACKSLASH => {
116 |             BACKSLASH => {
    |             --------- matches any value
...
147 |             _ => cur.push(w.get()),
    |             ^ unreachable pattern
error: could not compile `std` due to 19 previous errors
Build completed unsuccessfully in 0:00:22
