plain
    Checking addr2line v0.19.0
error[E0405]: cannot find trait `Pattern` in this scope
   --> library/std/src/sys/windows/os_str.rs:241:32
    |
241 |     pub fn strip_prefix<'a, P: Pattern<'a>>(&'a self, prefix: P) -> Option<&'a Slice> {
    |
help: consider importing one of these items
    |
3   + use alloc::str::pattern::Pattern;
---

error[E0405]: cannot find trait `Pattern` in this scope
   --> library/std/src/sys/windows/os_str.rs:251:30
    |
251 |     pub fn split_once<'a, P: Pattern<'a>>(&'a self, delimiter: P) -> Option<(&'a str, &'a Slice)> {
    |
help: consider importing one of these items
    |
3   + use alloc::str::pattern::Pattern;
