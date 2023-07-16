plain
[01:23:58] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:58]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:951:12
[01:24:04]     |
[01:24:04] 951 |         fm.next_line(BytePos(0));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:955:12
[01:24:04]     |
[01:24:04] 955 |         fm.next_line(BytePos(10));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:957:12
[01:24:04]     |
[01:24:04] 957 |         fm.next_line(BytePos(12));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:968:12
[01:24:04]     |
[01:24:04] 968 |         fm.next_line(BytePos(0));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:969:12
[01:24:04]     |
[01:24:04] 969 |         fm.next_line(BytePos(10));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:970:12
[01:24:04]     |
[01:24:04] 970 |         fm.next_line(BytePos(2));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:982:13
[01:24:04]     |
[01:24:04] 982 |         fm1.next_line(BytePos(0));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:983:13
[01:24:04]     |
[01:24:04] 983 |         fm1.next_line(BytePos(12));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:984:13
[01:24:04]     |
[01:24:04] 984 |         fm2.next_line(fm2.start_pos);
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:985:13
[01:24:04]     |
[01:24:04] 985 |         fm3.next_line(fm3.start_pos);
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]    --> libsyntax/codemap.rs:986:13
[01:24:04]     |
[01:24:04] 986 |         fm3.next_line(fm3.start_pos + BytePos(12));
[01:24:04]     |
[01:24:04]     = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1046:13
[01:24:04]      |
[01:24:04] 1046 |         fm1.next_line(BytePos(0));
[01:24:04]      |
[01:24:04]      = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1047:13
[01:24:04]      |
[01:24:04] 1047 |         fm1.next_line(BytePos(28));
[01:24:04]      |
[01:24:04]      = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1048:13
[01:24:04]      |
[01:24:04] 1048 |         fm2.next_line(fm2.start_pos);
[01:24:04]      |
[01:24:04]      = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `next_line` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1049:13
[01:24:04]      |
[01:24:04] 1049 |         fm2.next_line(fm2.start_pos + BytePos(20));
[01:24:04]      |
[01:24:04]      = help: did you mean `get_line`?
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1051:13
[01:24:04]      |
[01:24:04] 1051 |         fm1.record_multibyte_char(BytePos(3), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1052:13
[01:24:04]      |
[01:24:04] 1052 |         fm1.record_multibyte_char(BytePos(9), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1053:13
[01:24:04]      |
[01:24:04] 1053 |         fm1.record_multibyte_char(BytePos(12), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1054:13
[01:24:04]      |
[01:24:04] 1054 |         fm1.record_multibyte_char(BytePos(15), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1055:13
[01:24:04]      |
[01:24:04] 1055 |         fm1.record_multibyte_char(BytePos(18), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1056:13
[01:24:04]      |
[01:24:04] 1056 |         fm2.record_multibyte_char(fm2.start_pos + BytePos(10), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1057:13
[01:24:04]      |
[01:24:04] 1057 |         fm2.record_multibyte_char(fm2.start_pos + BytePos(13), 3);
[01:24:04] 
[01:24:04] 
[01:24:04] error[E0599]: no method named `record_multibyte_char` found for type `std::rc::Rc<syntax_pos::FileMap>` in the current scope
[01:24:04]     --> libsyntax/codemap.rs:1058:13
[01:24:04]      |
[01:24:04] 1058 |         fm2.record_multibyte_char(fm2.start_pos + BytePos(18), 3);
[01:24:04] 
[01:24:08] error: aborting due to 23 previous errors
[01:24:08] 
[01:24:08] For more information about this error, try `rustc --explain E0599`.
[01:24:08] For more information about this error, try `rustc --explain E0599`.
[01:24:08] error: Could not compile `syntax`.
[01:24:08] 
[01:24:08] To learn more, run the command again with --verbose.
[01:24:08] 
[01:24:08] 
[01:24:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:24:08] 
[01:24:08] 
[01:24:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:08] Build completed unsuccessfully in 0:39:18
[01:24:08] Build completed unsuccessfully in 0:39:18
[01:24:08] make: *** [check] Error 1
[01:24:08] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d70ef0a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
