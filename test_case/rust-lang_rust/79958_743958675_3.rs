text
===================================================================================================

These tests fail with `-Z symbol-mangling-version=v0` (without `-Z instrument-coverage`).
`-Z instrument-coverage` forces `v0` and requires it.
Some of these tests are testing `legacy` symbol mangling, so they will never be compatible with
`-Z instrument-coverage`.

src/test/ui/const-generics/issues/issue-62579-no-match.rs
src/test/ui/const-generics/slice-const-param.rs
src/test/ui/const-generics/type-dependent/issue-71348.rs
src/test/ui/panics/issue-47429-short-backtraces.rs
src/test/ui/privacy/privacy2.rs
src/test/ui/privacy/privacy3.rs
src/test/ui/symbol-names/basic.rs#legacy
src/test/ui/symbol-names/impl1.rs#legacy
src/test/ui/symbol-names/issue-60925.rs#legacy
src/test/ui/symbol-names/issue-75326.rs#legacy
src/test/ui/type_length_limit.rs

