plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking ena v0.14.0
    Checking polonius-engine v0.12.1
    Checking tracing-log v0.1.2
    Checking rand_core v0.5.1
error[E0412]: cannot find type `EscapeWarning` in this scope
   --> compiler/rustc_lexer/src/unescape/tests.rs:105:56
103 |     fn check(
103 |     fn check(
    |             - help: you might be missing a type parameter: `<EscapeWarning>`
104 |         literal: &str,
105 |         expected: &[(Range<usize>, Result<Result<char, EscapeWarning>, EscapeError>)],

    Checking aho-corasick v0.7.13
   Compiling psm v0.1.11
   Compiling stacker v0.1.12
   Compiling stacker v0.1.12
error[E0599]: no variant or associated item named `UnskippedWhitespaceAfterEscapedNewline` found for enum `unescape::EscapeError` in the current scope
   --> compiler/rustc_lexer/src/unescape/tests.rs:115:37
    |
115 |             (0..5, Err(EscapeError::UnskippedWhitespaceAfterEscapedNewline)),
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ variant or associated item not found in `unescape::EscapeError`
   ::: compiler/rustc_lexer/src/unescape.rs:12:1
    |
12  | pub enum EscapeError {
12  | pub enum EscapeError {
    | -------------------- variant or associated item `UnskippedWhitespaceAfterEscapedNewline` not found here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0599.
For more information about an error, try `rustc --explain E0412`.
