rust
// This relies on all repr(rust) union fields being located at offset 0,
// which is currently not guaranteed, see:
// * [link to unions RFC 1.2]
// * https://github.com/rust-rfcs/unsafe-code-guidelines/issues/13
