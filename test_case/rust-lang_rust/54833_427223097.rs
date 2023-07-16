plain
[00:05:49]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:54]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:54]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:01]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:12] error[E0599]: no method named `conains` found for type `std::vec::Vec<parse::parser::TokenType>` in the current scope
[00:06:12]     |
[00:06:12]     |
[00:06:12] 867 |         debug_assert!(!self.expected_tokens.conains(&token),
[00:06:12]     |
[00:06:12]     = help: did you mean `contains`?
[00:06:12] 
[00:06:12] 
[00:06:12] error[E0277]: `syntax_pos::symbol::keywords::Keyword` doesn't implement `std::fmt::Debug`
[00:06:12]     |
[00:06:12]     |
[00:06:12] 869 |                       kw, self.expected_tokens);
[00:06:12]     |                       ^^ `syntax_pos::symbol::keywords::Keyword` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
[00:06:12]     = help: the trait `std::fmt::Debug` is not implemented for `syntax_pos::symbol::keywords::Keyword`
[00:06:12]     = note
travis_time:end:06d37088:start=1538704242217519360,finish=1538704616990765253,duration=374773245893

---
travis_time:end:01a2e69c:start=1538704617407530482,finish=1538704617412070228,duration=4539746
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19dddbec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0028f31a
travis_time:start:0028f31a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29c04c3a
$ dmesg | grep -i kill
