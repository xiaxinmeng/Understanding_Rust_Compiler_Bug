",
[01:05:51]     "    ///",
[01:05:51]     "    /// [`rchunks_mut`]: #method.rchunks_mut",
[01:05:51]     "    /// [`chunks_exact_mut`]: #method.chunks_exact_mut"
[01:05:51] warning: `[chunks_mut]` cannot be resolved, ignoring it...
[01:05:51]    --> src/libcore/slice/mod.rs:899:52
[01:05:51]     |
[01:05:51] 899 |     /// resulting code better than in the case of [`chunks_mut`].
---
[01:05:56]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[01:05:56]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[01:06:00]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:06:00]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::passes::collect_intra_doc_links: Attributes {
[01:06:06]     doc_strings: [
[01:06:06]         SugaredDoc(
[01:06:06]             0,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:975:5: 975:39,
[01:06:06]             " Takes the contained value out."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             1,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:976:5: 976:8,
[01:06:06]             ""
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             1,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:977:5: 977:73,
[01:06:06]             " This method is primarily intended for moving out values in drop."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             2,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:978:5: 978:76,
[01:06:06]             " Instead of using [`ManuallyDrop::drop`] to manually drop the value,"
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             3,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:979:5: 979:78,
[01:06:06]             " you can use this method to take the value and use it however desired."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             4,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:980:5: 980:90,
[01:06:06]             " `Drop` will be invoked on the returned value following normal end-of-scope rules."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             5,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:981:5: 981:8,
[01:06:06]             ""
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             5,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:982:5: 982:98,
[01:06:06]             " If you have ownership of the container, you can use [`ManuallyDrop::into_inner`] instead."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             6,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:983:5: 983:8,
[01:06:06]             ""
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             6,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:984:5: 984:17,
[01:06:06]             " # Safety"
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             7,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:985:5: 985:8,
[01:06:06]             ""
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             7,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:986:5: 986:99,
[01:06:06]             " This function semantically moves out the contained value without preventing further usage."
[01:06:06]         ),
[01:06:06]         SugaredDoc(
[01:06:06]             8,
[01:06:06]             /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:987:5: 987:93,
[01:06:06]             " It is up to the user of this method to ensure that this container is not used again."
[01:06:06]     ],
[01:06:06]     other_attrs: [
[01:06:06]         Attribute {
[01:06:06]             id: AttrId(
[01:06:06]             id: AttrId(
[01:06:06]                 70565
[01:06:06]             ),
[01:06:06]             style: Outer,
[01:06:06]             path: path(must_use),
[01:06:06]             tokens: TokenStream {
[01:06:06]                 kind: Stream(
[01:06:06]                         TokenStream {
[01:06:06]                             kind: Tree(
[01:06:06]                                 Token(
[01:06:06]                                 Token(
[01:06:06]                                     /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:988:16: 988:17,
[01:06:06]                                     Eq
[01:06:06]                             )
[01:06:06]                         },
[01:06:06]                         TokenStream {
[01:06:06]                             kind: Tree(
[01:06:06]                             kind: Tree(
[01:06:06]                                 Token(
[01:06:06]                                     /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:988:18: 988:89,
[01:06:06]                                     Literal(
[01:06:06]                                         Str_(
[01:06:06]                                             if you don't need the value, you can use `ManuallyDrop::drop` instead
[01:06:06]                                         None
[01:06:06]                                     )
[01:06:06]                                 )
[01:06:06]                             )
[01:06:06]                             )
[01:06:06]                         }
[01:06:06]                     ]
[01:06:06]                 )
[01:06:06]             },
[01:06:06]             is_sugared_doc: false,
[01:06:06]             span: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:988:5: 988:90
[01:06:06]         Attribute {
[01:06:06]             id: AttrId(
[01:06:06]                 70566
[01:06:06]             ),
[01:06:06]             ),
[01:06:06]             style: Outer,
[01:06:06]             path: path(unstable),
[01:06:06]             tokens: TokenStream {
[01:06:06]                 kind: Tree(
[01:06:06]                     Delimited(
[01:06:06]                         DelimSpan {
[01:06:06]                             open: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:15: 989:16,
[01:06:06]                             close: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:63: 989:64
[01:06:06]                         Paren,
[01:06:06]                         ThinTokenStream(
[01:06:06]                             Some(
[01:06:06]                                 [
[01:06:06]                                 [
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:16: 989:23,
[01:06:06]                                                 Ident(
[01:06:06]                                                     feature#0,
[01:06:06]                                                 )
[01:06:06]                                             )
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:24: 989:25,
[01:06:06]                                                 Eq
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:26: 989:46,
[01:06:06]                                                 Literal(
[01:06:06]                                                     Str_(
[01:06:06]                                                     ),
[01:06:06]                                                     None
[01:06:06]                                                 )
[01:06:06]                                             )
[01:06:06]                                             )
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:46: 989:47,
[01:06:06]                                             )
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:48: 989:53,
[01:06:06]                                                 Ident(
[01:06:06]                                                     issue#0,
[01:06:06]                                                 )
[01:06:06]                                             )
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:54: 989:55,
[01:06:06]                                                 Eq
[01:06:06]                                         )
[01:06:06]                                     },
[01:06:06]                                     TokenStream {
[01:06:06]                                         kind: Tree(
[01:06:06]                                         kind: Tree(
[01:06:06]                                             Token(
[01:06:06]                                                 /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:56: 989:63,
[01:06:06]                                                 Literal(
[01:06:06]                                                     Str_(
[01:06:06]                                                     ),
[01:06:06]                                                     None
[01:06:06]                                                 )
[01:06:06]                                             )
---
[01:06:06]                         )
[01:06:06]                     )
[01:06:06]                 )
[01:06:06]             },
[01:06:06]             is_sugared_doc: false,
[01:06:06]             span: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:989:5: 989:65
[01:06:06]         Attribute {
[01:06:06]             id: AttrId(
[01:06:06]                 70567
[01:06:06]             ),
[01:06:06]             ),
[01:06:06]             style: Outer,
[01:06:06]             path: path(inline),
[01:06:06]             tokens: TokenStream {
[01:06:06]                 kind: Empty
[01:06:06]             is_sugared_doc: false,
[01:06:06]             is_sugared_doc: false,
[01:06:06]             span: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:990:5: 990:14
[01:06:06]     ],
[01:06:06]     cfg: None,
[01:06:06]     span: Some(
[01:06:06]     span: Some(
[01:06:06]         /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:975:5: 975:39
[01:06:06]     links: [],
[01:06:06]     inner_docs: false
[01:06:06] }
[01:06:06] }
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::passes::collect_intra_doc_links: /rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs:975:5: 987:93
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::clean: SourceNotAvailable { filename: Real("/rustc/4c970306504a0e168de1e4dab641532834acc0ba/src/libcore/mem.rs") }
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::passes::collect_intra_doc_links: ""
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::passes::collect_intra_doc_links: [
[01:06:06]     " Takes the contained value out.",
[01:06:06]     "",
[01:06:06]     " This method is primarily intended for moving out values in drop.",
[01:06:06]     " Instead of using [`ManuallyDrop::drop`] to manually drop the value,"
[01:06:06] ERROR 2018-12-10T21:38:11Z: rustdoc::passes::collect_intra_doc_links: []
[01:06:06] thread '<unnamed>' panicked at 'could not find markdown line in source', src/libcore/option.rs:1008:5
[01:06:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:06] 
[01:06:06] 
[01:06:06] error: internal compiler error: unexpected panic
[01:06:06] 
[01:06:06] error: Unrecognized option: 'markdown-css'
[01:06:06] 
[01:06:06] error: Could not document `std`.
[01:06:06] 
[01:06:06] Caused by:
[01:06:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std src/libstd/lib.rs --color always --target i686-unknown-linux-gnu -o /checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/liballoc-7882740d30281eea.rmeta --extern compiler_builtins=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libcompiler_builtins-d49e06860bf93b9a.rmeta --extern core=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libcore-259373ac6295e997.rmeta --extern libc=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/liblibc-19d97af701d07897.rmeta --extern panic_abort=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libpanic_abort-89481050dda3e751.rmeta --extern panic_unwind=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libpanic_unwind-4b52e746f7bb8f65.rmeta --extern profiler_builtins=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libprofiler_builtins-1e4fb015ef827b35.rmeta --extern unwind=/checkout/obj/build/i686-unknown-linux-gnu/stage1-std/i686-unknown-linux-gnu/release/deps/libunwind-2867f714ffeb2155.rmeta` (exit code: 1)
[01:06:06] 
[01:06:06] 
[01:06:06] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[01:06:06] 
[01:06:06] 
[01:06:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build i686-unknown-linux-gnu --host i686-unknown-linux-gnu --target i686-unknown-linux-gnu
[01:06:06] Build completed unsuccessfully in 1:00:03
---
travis_time:end:0db5af6c:start=1544477893588768633,finish=1544477893595555983,duration=6787350
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a0ecbf4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01d6b85e
travis_time:start:01d6b85e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:068f9df7
$ dmesg | grep -i kill
