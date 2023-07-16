 rust
// In contrast to the NaCl SDK PNaCl toolchain, we do things a little
// differently.
// `pnacl-clang` or `pnacl-ld` run all optimization passes (set by
// -O[0-3sz]) after linking. This isn't such a problem when you have a
// compiler (ie clang) that generates reasonably sized modules to begin
// with, or when you don't have a test suite with 1500+ separate binary
// tests that you'd like to run and complete sometime before you
// retire. Rust isn't so lucky.
// So to make things more manageable, Rust runs the optimization passes
// on each of its modules as per usual, and uses LLVM's LTO passes for,
// well, LTO, with the IR simplification passes before and
// after. However, `pnacl-clang`s behavior is unchanged, which means we
// need to run the regular optimization passes on the current crate's
// link deps.
