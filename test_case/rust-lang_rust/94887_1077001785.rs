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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   |
   |
58 | impl<'a> PrefixParser<'a> {
   |          ^^^^^^^^^^^^ expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `LEN`
   |
   |
53 | struct PrefixParser<'a, const LEN: usize> {
help: add missing generic argument
   |
   |
58 | impl<'a> PrefixParser<'a, LEN> {


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   |
   |
84 | impl<'a> PrefixParserSlice<'a, '_> {
   |          ^^^^^^^^^^^^^^^^^ expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `LEN`
   |
   |
79 | struct PrefixParserSlice<'a, 'b, const LEN: usize> {
help: add missing generic argument
   |
   |
84 | impl<'a> PrefixParserSlice<'a, '_, LEN> {


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
   |
   |
74 |     fn as_slice(&self) -> PrefixParserSlice<'a, '_> {
   |                           ^^^^^^^^^^^^^^^^^ expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `LEN`
   |
   |
79 | struct PrefixParserSlice<'a, 'b, const LEN: usize> {
help: add missing generic argument
   |
   |
74 |     fn as_slice(&self) -> PrefixParserSlice<'a, '_, LEN> {

For more information about this error, try `rustc --explain E0107`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:17
