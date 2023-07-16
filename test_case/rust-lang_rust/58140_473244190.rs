plain
[01:39:08] test [pretty] pretty/issue-19077.rs ... ok
[01:39:08] test [pretty] pretty/issue-25031.rs ... ok
[01:39:08] test [pretty] pretty/issue-31073.rs ... ok
[01:39:08] test [pretty] pretty/issue-4264.rs ... FAILED
[01:39:08] ERROR 2019-03-15T10:58:55Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  (($crate::fmt::format as\n                       for<\'r> fn(std::fmt::Arguments<\'r>) -> std::string::String {std::fmt::format})(((<$crate::fmt::Arguments>::new_v1\n                                                                                                           as\n                                                                                                           fn(&[&str], &[std::fmt::ArgumentV1<\'_>]) -> std::fmt::Arguments<\'_> {std::fmt::Arguments<\'_>::new_v1})((&([(\"test\"\n                                                                                                                                                                                                                          as\n                                                                                                                                                                                                                          &\'static str)]\n                                                                                                                                                                                                                        as\n                                                                                                                                                                                                                        [&str; 1])\n                                                                                                                                                                                                                      as\n                                                                                                                                                                                                                      &[&str; 1]),\n                                                                                                                                                                                                                  (&(match (()\n                                                                                                                                                                                                                               as\n                                                                                                                                                                                                                               ())\n                                                                                                                                                                                                                         {\n                                                                                                                                                                                                                         ()\n                                                                                                                                                                                                                         =>\n                                                                                                                                                                                                                         ([]\n                                                                                                                                                                                                                             as\n                                                                                                                                                                                                                             [std::fmt::ArgumentV1<\'_>; 0]),\n                                                                                                                                                                                                                     }\n                                                                                                                                                                                                                        as\n                                                                                                                                                                                                                        [std::fmt::ArgumentV1<\'_>; 0])\n                                                                                                                                                                                                                      as\n                                                                                                                                                                                                                      &[std::fmt::ArgumentV1<\'_>; 0]))\n                                                                                                          as\n                                                                                                          std::fmt::Arguments<\'_>))\n                      as std::string::String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\nactual:\n------------------------------------------\n#[prelude_import]\nuse ::std::prelude::v1::*;\n#[macro_use]\nextern crate std;\n// pretty-compare-only\n// pretty-mode:hir,typed\n// pp-exact:issue-4264.pp\n\n// #4264 fixed-length vector types\n\npub fn foo(_: [i32; (3 as usize)]) ({ } as ())\n\npub fn bar() ({\n                  const FOO: usize = ((5 as usize) - (4 as usize) as usize);\n                  let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);\n\n                  let _ =\n                      (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])\n                            as &[i32; 3]) as *const _ as *const [i32; 3]) as\n                          *const [i32; (3 as usize)] as *const [i32; 3]);\n\n\n\n\n\n\n\n\n\n                  (($crate::fmt::format as\n                       for<\'r> fn(std::fmt::Arguments<\'r>) -> std::string::String {std::fmt::format})(((<$crate::fmt::Arguments>::new_v1\n                                                                                                           as\n                                                                                                           fn(&[&str], &[std::fmt::ArgumentV1<\'_>]) -> std::fmt::Arguments<\'_> {std::fmt::Arguments::<\'_>::new_v1})((&([(\"test\"\n                                                                                                                                                                                                                            as\n                                                                                                                                                                                                                            &\'static str)]\n                                                                                                                                                                                                                          as\n                                                                                                                                                                                                                          [&str; 1])\n                                                                                                                                                                                                                        as\n                                                                                                                                                                                                                        &[&str; 1]),\n                                                                                                                                                                                                                    (&(match (()\n                                                                                                                                                                                                                                 as\n                                                                                                                                                                                                                                 ())\n                                                                                                                                                                                                                           {\n                                                                                                                                                                                                                           ()\n                                                                                                                                                                                                                           =>\n                                                                                                                                                                                                                           ([]\n                                                                                                                                                                                                                               as\n                                                                                                                                                                                                                               [std::fmt::ArgumentV1<\'_>; 0]),\n                                                                                                                                                                                                                       }\n                                                                                                                                                                                                                          as\n                                                                                                                                                                                                                          [std::fmt::ArgumentV1<\'_>; 0])\n                                                                                                                                                                                                                        as\n                                                                                                                                                                                                                        &[std::fmt::ArgumentV1<\'_>; 0]))\n                                                                                                          as\n                                                                                                          std::fmt::Arguments<\'_>))\n                      as std::string::String);\n              } as ())\npub type Foo = [i32; (3 as usize)];\npub struct Bar {\n    pub x: [i32; (3 as usize)],\n}\npub struct TupleBar([i32; (4 as usize)]);\npub enum Baz { BazVariant([i32; (5 as usize)]), }\npub fn id<T>(x: T) -> T ({ (x as T) } as T)\npub fn use_id() ({\n                     let _ =\n                         ((id::<[i32; (3 as usize)]> as\n                              fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1\n                                                                               as\n                                                                               i32),\n                                                                           (2\n                                                                               as\n                                                                               i32),\n                                                                           (3\n                                                                               as\n                                                                               i32)]\n                                                                             as\n                                                                             [i32; 3]))\n                             as [i32; 3]);\n                 } as ())\nfn main() ({ } as ())\n\n------------------------------------------\n\n"
[01:39:08] test [pretty] pretty/lifetime.rs ... ok
[01:39:08] test [pretty] pretty/let.rs ... ok
[01:39:08] test [pretty] pretty/match-block-expr.rs ... ok
[01:39:08] test [pretty] pretty/match-naked-expr-medium.rs ... ok
---
[01:39:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:39:09] error: pretty-printed source does not match expected source
[01:39:09] expected:
[01:39:09] ------------------------------------------
[01:39:09] #[prelude_import]
[01:39:09] use ::std::prelude::v1::*;
[01:39:09] #[macro_use]
[01:39:09] extern crate std;
[01:39:09] // pretty-compare-only
[01:39:09] // pretty-mode:hir,typed
[01:39:09] // pp-exact:issue-4264.pp
[01:39:09] 
[01:39:09] // #4264 fixed-length vector types
[01:39:09] 
[01:39:09] pub fn foo(_: [i32; (3 as usize)]) ({ } as ())
[01:39:09] pub fn bar() ({
[01:39:09] pub fn bar() ({
[01:39:09]                   const FOO: usize = ((5 as usize) - (4 as usize) as usize);
[01:39:09]                   let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);
[01:39:09] 
[01:39:09]                   let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);
[01:39:09]                   let _ =
[01:39:09]                   let _ =
[01:39:09]                       (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])
[01:39:09]                             as &[i32; 3]) as *const _ as *const [i32; 3]) as
[01:39:09]                           *const [i32; (3 as usize)] as *const [i32; 3]);
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09]                   (($crate::fmt::format as
[01:39:09]                        for<'r> fn(std::fmt::Arguments<'r>) -> std::string::String {std::fmt::format})(((<$crate::fmt::Arguments>::new_v1
[01:39:09]                                                                                                            as
[01:39:09]                                                                                                            fn(&[&str], &[std::fmt::ArgumentV1<'_>]) -> std::fmt::Arguments<'_> {std::fmt::Arguments<'_>::new_v1})((&([("test"
[01:39:09]                                                                                                                                                                                                                           &'static str)]
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         [&str; 1])
[01:39:09]                                                                                                                                                                                                                       as
[01:39:09]                                                                                                                                                                                                                       &[&str; 1]),
[01:39:09]                                                                                                                                                                                                                   (&(match (()
[01:39:09]                                                                                                                                                                                                                                ())
[01:39:09]                                                                                                                                                                                                                          {
[01:39:09]                                                                                                                                                                                                                          ()
[01:39:09]                                                                                                                                                                                                                          =>
[01:39:09]                                                                                                                                                                                                                          =>
[01:39:09]                                                                                                                                                                                                                          ([]
[01:39:09]                                                                                                                                                                                                                              as
[01:39:09]                                                                                                                                                                                                                              [std::fmt::ArgumentV1<'_>; 0]),
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         [std::fmt::ArgumentV1<'_>; 0])
[01:39:09]                                                                                                                                                                                                                       as
[01:39:09]                                                                                                                                                                                                                       &[std::fmt::ArgumentV1<'_>; 0]))
[01:39:09]                                                                                                           std::fmt::Arguments<'_>))
[01:39:09]                       as std::string::String);
[01:39:09]                       as std::string::String);
[01:39:09]               } as ())
[01:39:09] pub type Foo = [i32; (3 as usize)];
[01:39:09] pub struct Bar {
[01:39:09]     pub x: [i32; (3 as usize)],
[01:39:09] }
[01:39:09] pub struct TupleBar([i32; (4 as usize)]);
[01:39:09] pub enum Baz { BazVariant([i32; (5 as usize)]), }
[01:39:09] pub fn id<T>(x: T) -> T ({ (x as T) } as T)
[01:39:09] pub fn use_id() ({
[01:39:09]                      let _ =
[01:39:09]                          ((id::<[i32; (3 as usize)]> as
[01:39:09]                               fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1
[01:39:09]                                                                                i32),
[01:39:09]                                                                            (2
[01:39:09]                                                                                as
[01:39:09]                                                                                i32),
[01:39:09]                                                                                i32),
[01:39:09]                                                                            (3
[01:39:09]                                                                                as
[01:39:09]                                                                                i32)]
[01:39:09]                                                                              as
[01:39:09]                                                                              [i32; 3]))
[01:39:09]                              as [i32; 3]);
[01:39:09]                  } as ())
[01:39:09] fn main() ({ } as ())
[01:39:09] ------------------------------------------
[01:39:09] actual:
[01:39:09] ------------------------------------------
[01:39:09] ------------------------------------------
[01:39:09] #[prelude_import]
[01:39:09] use ::std::prelude::v1::*;
[01:39:09] #[macro_use]
[01:39:09] extern crate std;
[01:39:09] // pretty-compare-only
[01:39:09] // pretty-mode:hir,typed
[01:39:09] // pp-exact:issue-4264.pp
[01:39:09] 
[01:39:09] // #4264 fixed-length vector types
[01:39:09] 
[01:39:09] pub fn foo(_: [i32; (3 as usize)]) ({ } as ())
[01:39:09] pub fn bar() ({
[01:39:09] pub fn bar() ({
[01:39:09]                   const FOO: usize = ((5 as usize) - (4 as usize) as usize);
[01:39:09]                   let _: [(); (FOO as usize)] = ([(() as ())] as [(); 1]);
[01:39:09] 
[01:39:09]                   let _: [(); (1 as usize)] = ([(() as ())] as [(); 1]);
[01:39:09]                   let _ =
[01:39:09]                   let _ =
[01:39:09]                       (((&([(1 as i32), (2 as i32), (3 as i32)] as [i32; 3])
[01:39:09]                             as &[i32; 3]) as *const _ as *const [i32; 3]) as
[01:39:09]                           *const [i32; (3 as usize)] as *const [i32; 3]);
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09]                   (($crate::fmt::format as
[01:39:09]                        for<'r> fn(std::fmt::Arguments<'r>) -> std::string::String {std::fmt::format})(((<$crate::fmt::Arguments>::new_v1
[01:39:09]                                                                                                            as
[01:39:09]                                                                                                            fn(&[&str], &[std::fmt::ArgumentV1<'_>]) -> std::fmt::Arguments<'_> {std::fmt::Arguments::<'_>::new_v1})((&([("test"
[01:39:09]                                                                                                                                                                                                                             &'static str)]
[01:39:09]                                                                                                                                                                                                                           as
[01:39:09]                                                                                                                                                                                                                           as
[01:39:09]                                                                                                                                                                                                                           [&str; 1])
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         &[&str; 1]),
[01:39:09]                                                                                                                                                                                                                     (&(match (()
[01:39:09]                                                                                                                                                                                                                                  ())
[01:39:09]                                                                                                                                                                                                                            {
[01:39:09]                                                                                                                                                                                                                            ()
[01:39:09]                                                                                                                                                                                                                            =>
[01:39:09]                                                                                                                                                                                                                            =>
[01:39:09]                                                                                                                                                                                                                            ([]
[01:39:09]                                                                                                                                                                                                                                as
[01:39:09]                                                                                                                                                                                                                                [std::fmt::ArgumentV1<'_>; 0]),
[01:39:09]                                                                                                                                                                                                                           as
[01:39:09]                                                                                                                                                                                                                           as
[01:39:09]                                                                                                                                                                                                                           [std::fmt::ArgumentV1<'_>; 0])
[01:39:09]                                                                                                                                                                                                                         as
[01:39:09]                                                                                                                                                                                                                         &[std::fmt::ArgumentV1<'_>; 0]))
[01:39:09]                                                                                                           std::fmt::Arguments<'_>))
[01:39:09]                       as std::string::String);
[01:39:09]                       as std::string::String);
[01:39:09]               } as ())
[01:39:09] pub type Foo = [i32; (3 as usize)];
[01:39:09] pub struct Bar {
[01:39:09]     pub x: [i32; (3 as usize)],
[01:39:09] }
[01:39:09] pub struct TupleBar([i32; (4 as usize)]);
[01:39:09] pub enum Baz { BazVariant([i32; (5 as usize)]), }
[01:39:09] pub fn id<T>(x: T) -> T ({ (x as T) } as T)
[01:39:09] pub fn use_id() ({
[01:39:09]                      let _ =
[01:39:09]                          ((id::<[i32; (3 as usize)]> as
[01:39:09]                               fn([i32; 3]) -> [i32; 3] {id::<[i32; 3]>})(([(1
[01:39:09]                                                                                i32),
[01:39:09]                                                                            (2
[01:39:09]                                                                                as
[01:39:09]                                                                                i32),
[01:39:09]                                                                                i32),
[01:39:09]                                                                            (3
[01:39:09]                                                                                as
[01:39:09]                                                                                i32)]
[01:39:09]                                                                              as
[01:39:09]                                                                              [i32; 3]))
[01:39:09]                              as [i32; 3]);
[01:39:09]                  } as ())
[01:39:09] fn main() ({ } as ())
[01:39:09] ------------------------------------------
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] thread '[pretty] pretty/issue-4264.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2043:9
[01:39:09] 
[01:39:09] 
[01:39:09] failures:
[01:39:09]     [pretty] pretty/issue-4264.rs
[01:39:09]     [pretty] pretty/issue-4264.rs
[01:39:09] 
[01:39:09] test result: FAILED. 52 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:39:09] 
[01:39:09] 
[01:39:09] 
[01:39:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:39:09] Build completed unsuccessfully in 0:01:05
[01:39:09] Makefile:50: recipe for target 'check-aux' failed
[01:39:09] make: *** [check-aux] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22013831
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 15 10:58:57 UTC 2019
---
travis_time:end:1256c27a:start=1552647539732052058,finish=1552647539740314112,duration=8262054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f75ada0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0180a498
travis_time:start:0180a498
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b21a948
$ dmesg | grep -i kill
