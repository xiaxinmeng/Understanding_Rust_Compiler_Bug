
% make check-stage1 TESTNAME=Regex_1
cfg: build triple x86_64-apple-darwin
cfg: host triples x86_64-apple-darwin
cfg: target triples x86_64-apple-darwin
cfg: enabling more debugging (CFG_ENABLE_DEBUG)
cfg: host for x86_64-apple-darwin is x86_64
cfg: os for x86_64-apple-darwin is apple-darwin
cfg: using ccache clang
cfg: including test rules
run doc-crate-regex [x86_64-apple-darwin]

running 1 test
test re::Regex_1 ... FAILED

failures:

---- re::Regex_1 stdout ----
    <anon>:6:18: 6:44 error: dlopen(/Users/fklock/Dev/Mozilla/rust-fixstage1/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-10a265ff-0.11-pre.dylib, 1): Symbol not found: __ZN11owned_slice19OwnedSlice$LT$T$GT$8as_slice10PTR_MARKER20h4ab916fef4e2db721Ba9v0.11.preE
      Referenced from: /Users/fklock/Dev/Mozilla/rust-fixstage1/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-10a265ff-0.11-pre.dylib
      Expected in: /Users/fklock/Dev/Mozilla/rust-fixstage1/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/libsyntax-5de2fc8e-0.11-pre.dylib
     in /Users/fklock/Dev/Mozilla/rust-fixstage1/objdir-check1-dbgopt/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-10a265ff-0.11-pre.dylib
    <anon>:6 #[phase(syntax)] extern crate regex_macros;
                              ^~~~~~~~~~~~~~~~~~~~~~~~~~
    task 're::Regex_1' failed at '~Any', /Users/fklock/Dev/Mozilla/rust-fixstage1/src/libsyntax/diagnostic.rs:79


failures:
    re::Regex_1

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
