plain
   Compiling miniz_oxide v0.4.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling object v0.26.2
   Compiling addr2line v0.16.0
error: expected one of `+`, `,`, `::`, `=`, or `>`, found `(`
     |
1833 | impl Path {
     |           - while parsing this item list starting here
...
...
2339 |     pub fn join_fold<P: AsRef<Path>(&self, path: P) -> PathBuf {
     |                                    ^ expected one of `+`, `,`, `::`, `=`, or `>`
2787 | }
2787 | }
     | - the item list ends here
error: unused import: `crate::fs`
  --> library/std/src/path.rs:70:5
   |
70 | use crate::fs;
70 | use crate::fs;
   |     ^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::io`
  --> library/std/src/path.rs:72:5
   |
72 | use crate::io;
72 | use crate::io;
   |     ^^^^^^^^^

error: unused import: `parse_prefix`
  --> library/std/src/path.rs:81:54
   |
81 | use crate::sys::path::{is_sep_byte, is_verbatim_sep, parse_prefix, MAIN_SEP_STR};


error[E0599]: no method named `is_dir` found for reference `&Path` in the current scope
     |
     |
2225 |             Err(_) if path.is_dir() => return Ok(()),
     |                            ^^^^^^ method not found in `&Path`

error[E0599]: no method named `is_dir` found for reference `&Path` in the current scope
     |
     |
2239 |             Err(_) if path.is_dir() => Ok(()),
     |                            ^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
   --> library/std/src/path.rs:620:47
    |
    |
620 |                 f.debug_list().entries(self.0.components()).finish()
    |                                               ^^^^^^^^^^ method not found in `&Path`

error[E0599]: no method named `iter` found for reference `&Path` in the current scope
    |
    |
800 |                 f.debug_list().entries(self.0.iter()).finish()
    |                                               ^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for mutable reference `&mut PathBuf` in the current scope
    --> library/std/src/path.rs:1235:30
     |
     |
1235 |             let comps = self.components();
     |                              ^^^^^^^^^^ method not found in `&mut PathBuf`
error[E0599]: no method named `components` found for mutable reference `&mut PathBuf` in the current scope
    --> library/std/src/path.rs:1250:35
     |
     |
1250 |             let prefix_len = self.components().prefix_remaining();
     |                                   ^^^^^^^^^^ method not found in `&mut PathBuf`

error[E0599]: no method named `into_path_buf` found for struct `Box<Path>` in the current scope
     |
1506 |         boxed.into_path_buf()
     |               ^^^^^^^^^^^^^ help: there is an associated function with a similar name: `to_path_buf`


error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1741:14
     |
1741 |         self.components() == other.components()
     |              ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1741:36
     |
     |
1741 |         self.components() == other.components()
     |                                    ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1759:38
     |
     |
1759 |         Some(compare_components(self.components(), other.components()))
     |                                      ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1759:58
     |
     |
1759 |         Some(compare_components(self.components(), other.components()))
     |                                                          ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1767:33
     |
     |
1767 |         compare_components(self.components(), other.components())
     |                                 ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&PathBuf` in the current scope
    --> library/std/src/path.rs:1767:53
     |
     |
1767 |         compare_components(self.components(), other.components())
     |                                                     ^^^^^^^^^^ method not found in `&PathBuf`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2000:14
     |
     |
2000 |         self.components().prefix
     |              ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2022:14
     |
     |
2022 |         self.components().has_root()
     |              ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2044:30
     |
     |
2044 |         let mut comps = self.components();
     |                              ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2110:14
     |
     |
2110 |         self.components().next_back().and_then(|p| match p {
     |              ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2153:25
     |
     |
2153 |         iter_after(self.components(), base.components())
     |                         ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2153:44
     |
     |
2153 |         iter_after(self.components(), base.components())
     |                                            ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2186:25
     |
     |
2186 |         iter_after(self.components(), base.components()).is_some()
     |                         ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2186:44
     |
     |
2186 |         iter_after(self.components(), base.components()).is_some()
     |                                            ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2213:25
     |
     |
2213 |         iter_after(self.components().rev(), child.components().rev()).is_some()
     |                         ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2213:51
     |
     |
2213 |         iter_after(self.components().rev(), child.components().rev()).is_some()
     |                                                   ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2847:14
     |
     |
2847 |         self.components() == other.components()
     |              ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2847:36
     |
     |
2847 |         self.components() == other.components()
     |                                    ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2854:31
     |
2854 |         for component in self.components() {
2854 |         for component in self.components() {
     |                               ^^^^^^^^^^ method not found in `&Path`

error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2867:38
     |
2867 |         Some(compare_components(self.components(), other.components()))
     |                                      ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2867:58
     |
     |
2867 |         Some(compare_components(self.components(), other.components()))
     |                                                          ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2875:33
     |
     |
2875 |         compare_components(self.components(), other.components())
     |                                 ^^^^^^^^^^ method not found in `&Path`
error[E0599]: no method named `components` found for reference `&Path` in the current scope
    --> library/std/src/path.rs:2875:53
     |
     |
2875 |         compare_components(self.components(), other.components())
     |                                                     ^^^^^^^^^^ method not found in `&Path`

error[E0599]: no method named `iter` found for reference `&'a PathBuf` in the current scope
     |
2941 |         self.iter()
2941 |         self.iter()
     |              ^^^^ method not found in `&'a PathBuf`

error[E0599]: no method named `iter` found for reference `&'a Path` in the current scope
     |
2951 |         self.iter()
2951 |         self.iter()
     |              ^^^^ method not found in `&'a Path`

error[E0599]: no method named `display` found for enum `Cow<'_, Path>` in the current scope
    |
    |
230 |     fmt::Display::fmt(&file.display(), fmt)
    |                             ^^^^^^^ method not found in `Cow<'_, Path>`

error[E0599]: no method named `is_dir` found for reference `&Path` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:289:70
    |
289 |                 exists = if Path::new(OsStr::from_bytes(DEBUG_PATH)).is_dir() {
    |                                                                      ^^^^^^ method not found in `&Path`

error[E0599]: no method named `is_file` found for struct `PathBuf` in the current scope
    --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:363:23
     |
363  |     if f != path && f.is_file() {
     |                       ^^^^^^^ method not found in `PathBuf`
    ::: library/std/src/path.rs:1128:1
     |
1128 | pub struct PathBuf {
1128 | pub struct PathBuf {
     | ------------------ method `is_file` not found for this

error[E0599]: no method named `is_file` found for struct `PathBuf` in the current scope
    --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:374:10
     |
374  |     if f.is_file() {
     |          ^^^^^^^ method not found in `PathBuf`
    ::: library/std/src/path.rs:1128:1
     |
1128 | pub struct PathBuf {
1128 | pub struct PathBuf {
     | ------------------ method `is_file` not found for this

error[E0599]: no method named `is_file` found for struct `PathBuf` in the current scope
    --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:386:14
     |
386  |         if f.is_file() {
     |              ^^^^^^^ method not found in `PathBuf`
    ::: library/std/src/path.rs:1128:1
     |
1128 | pub struct PathBuf {
1128 | pub struct PathBuf {
     | ------------------ method `is_file` not found for this

error[E0599]: no method named `is_file` found for reference `&Path` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:409:21
409 |         if filename.is_file() {
    |                     ^^^^^^^ method not found in `&Path`


error[E0599]: no method named `is_file` found for struct `PathBuf` in the current scope
    --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:417:14
     |
417  |         if f.is_file() {
     |              ^^^^^^^ method not found in `PathBuf`
    ::: library/std/src/path.rs:1128:1
     |
1128 | pub struct PathBuf {
1128 | pub struct PathBuf {
     | ------------------ method `is_file` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 43 previous errors
Build completed unsuccessfully in 0:00:21
