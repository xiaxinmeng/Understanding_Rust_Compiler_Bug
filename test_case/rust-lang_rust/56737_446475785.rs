plain
travis_time:end:0499a949:start=1544590199028490396,finish=1544590201310673674,duration=2282183278
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:03:58]     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:04:00]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:04:00]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:01]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:09] error[E0277]: `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be sent between threads safely
[00:04:09]    --> src/libsyntax/ext/tt/macro_rules.rs:396:13
[00:04:09] 396 |             expander,
[00:04:09] 396 |             expander,
[00:04:09]     |             ^^^^^^^^ `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be sent between threads safely
[00:04:09]     |
[00:04:09]     = help: within `(parse::token::Nonterminal, parse::token::LazyTokenStream)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>`
[00:04:09]     = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>>`
[00:04:09]     = note: required because it appears within the type `tokenstream::ThinTokenStream`
[00:04:09]     = note: required because it appears within the type `tokenstream::TokenTree`
[00:04:09]     = note: required because it appears within the type `parse::token::Nonterminal`
[00:04:09]     = note: required because it appears within the type `(parse::token::Nonterminal, parse::token::LazyTokenStream)`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<(parse::token::Nonterminal, parse::token::LazyTokenStream)>`
[00:04:09]     = note: required because it appears within the type `parse::token::Token`
[00:04:09]     = note: required because it appears within the type `ext::tt::quoted::TokenTree`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `alloc::raw_vec::RawVec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `std::vec::Vec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `ext::tt::macro_rules::MacroRulesMacroExpander`
[00:04:09]     = note: required for the cast to the object type `dyn ext::base::TTMacroExpander + std::marker::Send + std::marker::Sync`
[00:04:09] 
[00:04:09] error[E0277]: `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be shared between threads safely
[00:04:09]    --> src/libsyntax/ext/tt/macro_rules.rs:396:13
[00:04:09] 396 |             expander,
[00:04:09] 396 |             expander,
[00:04:09]     |             ^^^^^^^^ `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be shared between threads safely
[00:04:09]     |
[00:04:09]     = help: within `(parse::token::Nonterminal, parse::token::LazyTokenStream)`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>`
[00:04:09]     = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>>`
[00:04:09]     = note: required because it appears within the type `tokenstream::ThinTokenStream`
[00:04:09]     = note: required because it appears within the type `tokenstream::TokenTree`
[00:04:09]     = note: required because it appears within the type `parse::token::Nonterminal`
[00:04:09]     = note: required because it appears within the type `(parse::token::Nonterminal, parse::token::LazyTokenStream)`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<(parse::token::Nonterminal, parse::token::LazyTokenStream)>`
[00:04:09]     = note: required because it appears within the type `parse::token::Token`
[00:04:09]     = note: required because it appears within the type `ext::tt::quoted::TokenTree`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `alloc::raw_vec::RawVec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `std::vec::Vec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `ext::tt::macro_rules::MacroRulesMacroExpander`
[00:04:09]     = note: required for the cast to the object type `dyn ext::base::TTMacroExpander + std::marker::Send + std::marker::Sync`
[00:04:09] 
[00:04:09] error[E0277]: `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be sent between threads safely
[00:04:09]    --> src/libsyntax/ext/tt/macro_rules.rs:408:13
[00:04:09] 408 |             expander,
[00:04:09] 408 |             expander,
[00:04:09]     |             ^^^^^^^^ `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be sent between threads safely
[00:04:09]     |
[00:04:09]     = help: within `(parse::token::Nonterminal, parse::token::LazyTokenStream)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>`
[00:04:09]     = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>>`
[00:04:09]     = note: required because it appears within the type `tokenstream::ThinTokenStream`
[00:04:09]     = note: required because it appears within the type `tokenstream::TokenTree`
[00:04:09]     = note: required because it appears within the type `parse::token::Nonterminal`
[00:04:09]     = note: required because it appears within the type `(parse::token::Nonterminal, parse::token::LazyTokenStream)`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<(parse::token::Nonterminal, parse::token::LazyTokenStream)>`
[00:04:09]     = note: required because it appears within the type `parse::token::Token`
[00:04:09]     = note: required because it appears within the type `ext::tt::quoted::TokenTree`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `alloc::raw_vec::RawVec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `std::vec::Vec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `ext::tt::macro_rules::MacroRulesMacroExpander`
[00:04:09]     = note: required for the cast to the object type `dyn ext::base::TTMacroExpander + std::marker::Send + std::marker::Sync`
[00:04:09] 
[00:04:09] error[E0277]: `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be shared between threads safely
[00:04:09]    --> src/libsyntax/ext/tt/macro_rules.rs:408:13
[00:04:09] 408 |             expander,
[00:04:09] 408 |             expander,
[00:04:09]     |             ^^^^^^^^ `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>` cannot be shared between threads safely
[00:04:09]     |
[00:04:09]     = help: within `(parse::token::Nonterminal, parse::token::LazyTokenStream)`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>`
[00:04:09]     = note: required because it appears within the type `std::option::Option<std::rc::Rc<std::vec::Vec<tokenstream::TokenStream>>>`
[00:04:09]     = note: required because it appears within the type `tokenstream::ThinTokenStream`
[00:04:09]     = note: required because it appears within the type `tokenstream::TokenTree`
[00:04:09]     = note: required because it appears within the type `parse::token::Nonterminal`
[00:04:09]     = note: required because it appears within the type `(parse::token::Nonterminal, parse::token::LazyTokenStream)`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<(parse::token::Nonterminal, parse::token::LazyTokenStream)>`
[00:04:09]     = note: required because it appears within the type `parse::token::Token`
[00:04:09]     = note: required because it appears within the type `ext::tt::quoted::TokenTree`
[00:04:09]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `alloc::raw_vec::RawVec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `std::vec::Vec<ext::tt::quoted::TokenTree>`
[00:04:09]     = note: required because it appears within the type `ext::tt::macro_rules::MacroRulesMacroExpander`
[00:04:09]     = note: required for the cast to the object type `dyn ext::base::TTMacroExpander + std::marker::Send + std::marker::Sync`
[00:04:09] error: aborting due to 4 previous errors
[00:04:09] 
[00:04:09] For more information about this error, try `rustc --explain E0277`.
[00:04:09] error: Could not compile `syntax`.
[00:04:09] error: Could not compile `syntax`.
[00:04:09] 
[00:04:09] To learn more, run the command again with --verbose.
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:09] Build completed unsuccessfully in 0:02:41
travis_time:end:10a468be:start=1544590210572672518,finish=1544590460214439220,duration=249641766702
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:01dcf37c:start=1544590460631927324,finish=1544590460638167393,duration=6240069
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11e0bde9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05d4745c
travis_time:start:05d4745c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0544ccda
$ dmesg | grep -i kill
