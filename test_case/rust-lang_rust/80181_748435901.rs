plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_912e9356-7182-4911-bd23-dd665cc45689
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=intra-doc-primitives
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_912e9356-7182-4911-bd23-dd665cc45689
GITHUB_REF=refs/pull/80181/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=432055603
GITHUB_RUN_NUMBER=21662
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-gawoha5c/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmp3q4dkvaupip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 9000/11181
.................................................................................................... 9100/11181
.........................................................................i......i................... 9200/11181
.................................................................................................... 9300/11181
............iiiiii..iiiiii.i........................................................................ 9400/11181
.................................................................................................... 9600/11181
.................................................................................................... 9700/11181
.................................................................................................... 9800/11181
.................................................................................................... 9900/11181
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.066 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....ii.ii..... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.96s

 finished in 2.027 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
error: `slice` is both a module and a builtin type
    |
    |
178 | //! [`std::slice`]: slice
    |                     ^^^^^ ambiguous link
    |
    = note: `-D broken-intra-doc-links` implied by `-D warnings`
help: to link to the module, prefix with `mod@`
    |
178 | //! [`std::slice`]: mod@slice
    |                     ^^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
    |
178 | //! [`std::slice`]: prim@slice


error: `slice` is both a module and a builtin type
   |
   |
69 | //! Note the documentation for the primitives [`str`] and [`[T]`][slice] (also
   |                                                                   ^^^^^ ambiguous link
   |
help: to link to the module, prefix with `mod@`
   |
69 | //! Note the documentation for the primitives [`str`] and [`[T]`][mod@slice] (also
   |                                                                   ^^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
   |
69 | //! Note the documentation for the primitives [`str`] and [`[T]`][prim@slice] (also


error: `slice` is both a module and a builtin type
   |
   |
71 | //! calls to methods on [`str`] and [`[T]`][slice] respectively, via [deref
   |                                             ^^^^^ ambiguous link
   |
help: to link to the module, prefix with `mod@`
   |
71 | //! calls to methods on [`str`] and [`[T]`][mod@slice] respectively, via [deref
   |                                             ^^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
   |
71 | //! calls to methods on [`str`] and [`[T]`][prim@slice] respectively, via [deref


error: `array` is both a module and a builtin type
    |
    |
114 | //! * [`[T; n]`][array] - An inline *array* with a fixed size at compile time.
    |                  ^^^^^ ambiguous link
    |
help: to link to the module, prefix with `mod@`
    |
114 | //! * [`[T; n]`][mod@array] - An inline *array* with a fixed size at compile time.
    |                  ^^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
    |
114 | //! * [`[T; n]`][prim@array] - An inline *array* with a fixed size at compile time.


error: `slice` is both a module and a builtin type
    |
    |
115 | //! * [`[T]`][slice] - A dynamically sized *slice* into any other kind of contiguous
    |               ^^^^^ ambiguous link
    |
help: to link to the module, prefix with `mod@`
    |
115 | //! * [`[T]`][mod@slice] - A dynamically sized *slice* into any other kind of contiguous
    |               ^^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
    |
115 | //! * [`[T]`][prim@slice] - A dynamically sized *slice* into any other kind of contiguous

error: aborting due to 5 previous errors

error: could not document `std`
error: could not document `std`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.50.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-891e9111d902c110.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-6348d408c51f1d5d.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-7149daa13c36fc69.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-834c6700638a0533.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-ff80b03a4c39786c.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-0f042af4b351b199.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-debb3bb860e894fe.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-ef91e0e8c5e5ea1e.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libobject-187ecd63efb75807.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-83100f2f6ebb03e8.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-68879d853b02b44d.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-4b1c21b54db9ab64.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-d9d7bae56b05dd33.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.50.0-nightly
  (a26ee5343
  2020-12-19)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.50.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:27:57
