
% DYLD_LIBRARY_PATH=x86_64-apple-darwin/stage2/lib x86_64-apple-darwin/stage2/bin/rustc /Users/fklock/Dev/Mozilla/rust-helphuon/src/test/compile-fail-fulldeps/syntax-extension-regex-invalid.rs -L x86_64-apple-darwin/test/compile-fail-fulldeps --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail-fulldeps/syntax-extension-regex-invalid.stage2-x86_64-apple-darwin.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail-fulldeps/syntax-extension-regex-invalid.stage2-x86_64-apple-darwin --cfg rtopt --cfg debug -O -L x86_64-apple-darwin/rt
dyld: lazy symbol binding failed: Symbol not found: __ZN2re5Regex3new20h605f1a58f435e390BZg9v0.11.preE
  Referenced from: /Users/fklock/Dev/Mozilla/rust-helphuon/objdir-dbgopt-check/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-10a265ff-0.11-pre.dylib
  Expected in: x86_64-apple-darwin/stage2/lib/libregex-1cae1bee-0.11-pre.dylib

dyld: Symbol not found: __ZN2re5Regex3new20h605f1a58f435e390BZg9v0.11.preE
  Referenced from: /Users/fklock/Dev/Mozilla/rust-helphuon/objdir-dbgopt-check/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libregex_macros-10a265ff-0.11-pre.dylib
  Expected in: x86_64-apple-darwin/stage2/lib/libregex-1cae1bee-0.11-pre.dylib

Trace/BPT trap: 5
