plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:14a9bba1
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:46:02] Documenting standalone (x86_64-unknown-linux-gnu)
[01:46:03] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[01:46:03] 
[01:46:03] 
[01:46:03] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/redirect.inc" "--markdown-no-toc" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc/book" "/checkout/src/doc/book/redirects/comments.md" "--markdown-css" "../rust.css"
[01:46:03] 
[01:46:03] 
[01:46:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:46:03] Build completed unsuccessfully in 1:39:44
---
travis_time:end:04884e86:start=1554773813479633488,finish=1554773813489506866,duration=9873378
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ed7e663
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.15981.!checkout!obj!build!x86_64-unknown-linux-gnu!stage2!bin!rustdoc
[New LWP 15983]
[New LWP 15982]
[New LWP 15981]
warning: Could not load shared library symbols for 7 libraries, e.g. /lib64/librt.so.1.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
Core was generated by `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc --html-after-co'.
Program terminated with signal SIGSEGV, Segmentation fault.
#0  0x00007fcf7abfbd22 in syntax_pos::span_encoding::SpanInterner::intern::h8d22e8e5d348883e () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libsyntax_pos-6122d0b7ba146acb.so
[Current thread is 1 (LWP 15983)]
#0  0x00007fcf7abfbd22 in syntax_pos::span_encoding::SpanInterner::intern::h8d22e8e5d348883e () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libsyntax_pos-6122d0b7ba146acb.so
#1  0x00007fcf7b4a6775 in syntax::parse::lexer::StringReader::advance_token::h03ec901defba58bc () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libsyntax-2a5682f1ec64e4dc.so
#2  0x00007fcf7b4a51f5 in syntax::parse::lexer::StringReader::new_or_buffered_errs::h8036fb10a9f3c13b () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libsyntax-2a5682f1ec64e4dc.so
#3  0x00005615aedf3d16 in render_with_highlighting () at src/librustdoc/html/highlight.rs:41
#4  0x00005615aeee3f2c in {{closure}}<rustdoc::html::markdown::LinkReplacer<rustdoc::html::markdown::HeadingLinks<pulldown_cmark::passes::Parser>>> () at src/librustdoc/html/markdown.rs:265
#5  try_with<core::cell::RefCell<core::option::Option<(core::option::Option<alloc::string::String>, alloc::string::String)>>,closure,core::option::Option<pulldown_cmark::parse::Event>> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:299
#6  with<core::cell::RefCell<core::option::Option<(core::option::Option<alloc::string::String>, alloc::string::String)>>,closure,core::option::Option<pulldown_cmark::parse::Event>> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:245
#7  0x00005615aed1576a in next<rustdoc::html::markdown::LinkReplacer<rustdoc::html::markdown::HeadingLinks<pulldown_cmark::passes::Parser>>> () at src/librustdoc/html/markdown.rs:189
#8  0x00005615aed18379 in next<rustdoc::html::markdown::CodeBlocks<rustdoc::html::markdown::LinkReplacer<rustdoc::html::markdown::HeadingLinks<pulldown_cmark::passes::Parser>>>> () at src/librustdoc/html/markdown.rs:471
#9  0x00005615aec9b4af in run<rustdoc::html::markdown::Footnotes<rustdoc::html::markdown::CodeBlocks<rustdoc::html::markdown::LinkReplacer<rustdoc::html::markdown::HeadingLinks<pulldown_cmark::passes::Parser>>>>> () at /cargo/registry/src/github.com-1ecc6299db9ec823/pulldown-cmark-0.1.2/src/html.rs:54
#10 0x00005615aed1c6ba in push_html<rustdoc::html::markdown::Footnotes<rustdoc::html::markdown::CodeBlocks<rustdoc::html::markdown::LinkReplacer<rustdoc::html::markdown::HeadingLinks<pulldown_cmark::passes::Parser>>>>> () at /cargo/registry/src/github.com-1ecc6299db9ec823/pulldown-cmark-0.1.2/src/html.rs:283
#11 fmt () at src/librustdoc/html/markdown.rs:710
#12 0x00007fcf7a2f827c in write () at src/libcore/fmt/mod.rs:1016
#13 0x00005615aecea315 in write_fmt<alloc::string::String> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libcore/fmt/mod.rs:195
#14 to_string<rustdoc::html::markdown::Markdown> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/liballoc/string.rs:2127
#15 render () at src/librustdoc/markdown.rs:81
#16 0x00005615aecc30b3 in main_args () at src/librustdoc/lib.rs:394
#17 0x00005615aeedf7c3 in {{closure}} () at src/librustdoc/lib.rs:98
#18 map<alloc::vec::Vec<alloc::string::String>,i32,closure> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libcore/option.rs:414
#19 {{closure}} () at src/librustdoc/lib.rs:98
#20 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc_interface/util.rs:186
#21 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc/ty/context.rs:1959
#22 try_with<core::cell::Cell<fn(&rustc_errors::diagnostic::Diagnostic)>,closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:299
#23 with<core::cell::Cell<fn(&rustc_errors::diagnostic::Diagnostic)>,closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:245
#24 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc/ty/context.rs:1951
#25 try_with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:299
#26 with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>>,closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/local.rs:245
#27 0x00005615aedae380 in with_thread_locals<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc/ty/context.rs:1943
#28 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc_interface/util.rs:186
#29 set<rustc_data_structures::sync::Lock<usize>,closure,i32> () at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#30 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc_interface/util.rs:182
#31 set<syntax_pos::Globals,closure,i32> () at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#32 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libsyntax/lib.rs:101
#33 set<syntax::Globals,closure,i32> () at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#34 0x00005615aeea2f09 in with_globals<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libsyntax/lib.rs:100
#35 0x00005615aeee94ab in {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc_interface/util.rs:181
#36 {{closure}}<closure,i32> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/librustc_interface/util.rs:159
#37 __rust_begin_short_backtrace<closure,()> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/sys_common/backtrace.rs:136
#38 0x00007fcf7a2da42a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:87
#39 0x00005615aeeea4a9 in try<(),std::panic::AssertUnwindSafe<closure>> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/panicking.rs:272
#40 catch_unwind<std::panic::AssertUnwindSafe<closure>,()> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/panic.rs:388
#41 {{closure}}<closure,()> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libstd/thread/mod.rs:469
#42 call_once<closure,()> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/libcore/ops/function.rs:231
#43 0x00007fcf7a2abe7f in call_once<(),FnBox<()>> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/liballoc/boxed.rs:702
#44 0x00007fcf7a2d9190 in call_once<(),alloc::boxed::Box<FnBox<()>>> () at /rustc/dc7e5bdf5f2bb2c403a36a306da90d58b188e3b2/src/liballoc/boxed.rs:702
#45 start_thread () at src/libstd/sys_common/thread.rs:14
#46 thread_start () at src/libstd/sys/unix/thread.rs:80
#47 0x00007fcf79e4283d in ?? ()
#48 0x0000000000000000 in ?? ()
travis_time:end:0ed7e663:start=1554773813496004220,finish=1554773815739123962,duration=2243119742
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2254e504
travis_time:start:2254e504
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22fa80a3
$ dmesg | grep -i kill
