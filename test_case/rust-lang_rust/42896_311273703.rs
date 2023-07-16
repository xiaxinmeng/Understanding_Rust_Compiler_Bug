rust
[00:34:05]    Compiling compiletest v0.0.0 (file:///checkout/src/tools/compiletest)
[00:34:05] error[E0308]: mismatched types
[00:34:05]    --> /checkout/src/tools/compiletest/src/header.rs:549:59
[00:34:05]     |
[00:34:05] 549 |   pub fn lldb_version_to_int(version_string: &str) -> isize {
[00:34:05]     |  ___________________________________________________________^
[00:34:05] 550 | |     let error_string = format!("Encountered LLDB version string with unexpected format: {}",
[00:34:05] 551 | |                                version_string);
[00:34:05] 552 | |     version_string.parse().expect(&error_string);
[00:34:05] 553 | | }
[00:34:05]     | |_^ expected isize, found ()
[00:34:05]     |
[00:34:05]     = note: expected type `isize`
[00:34:05]                found type `()`
[00:34:05] help: consider removing this semicolon:
[00:34:05]    --> /checkout/src/tools/compiletest/src/header.rs:552:49
[00:34:05]     |
[00:34:05] 552 |     version_string.parse().expect(&error_string);
[00:34:05]     |                                                 ^
[00:34:05] 
[00:34:06] error: aborting due to previous error(s)
[00:34:06] 
[00:34:06] error: Could not compile `compiletest`.
