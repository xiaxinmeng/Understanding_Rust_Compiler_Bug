plain
running 2 tests
FF
failures:

---- lib.rs - GenericParamDefKind::Type::synthetic (line 353) stdout ----
error: expected identifier, found keyword `impl`
  |
  |
3 | pub fn f<impl Trait: Trait>(_: impl Trait) {}


error: expected one of `,`, `:`, `=`, or `>`, found `Trait`
  |
  |
3 | pub fn f<impl Trait: Trait>(_: impl Trait) {}
  |               ^^^^^ expected one of `,`, `:`, `=`, or `>`
error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- lib.rs - GenericParamDefKind::Type::synthetic (line 347) stdout ----
error[E0405]: cannot find trait `Trait` in this scope
  |
  |
3 | pub fn f(_: impl Trait) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
Couldn't compile the test.

failures:
    lib.rs - GenericParamDefKind::Type::synthetic (line 347)
    lib.rs - GenericParamDefKind::Type::synthetic (line 353)
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:29:32
