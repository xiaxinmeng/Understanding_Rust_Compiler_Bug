plain
test [run-make] src/test/run-make/coverage-reports ... ok

failures:

---- [run-make] src/test/run-make/rlib-format-packed-bundled-libs-2 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
# Build strange-named dep.
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  native_dep.rs --crate-type=staticlib -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/native_dep.ext
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  rust_dep.rs --crate-type=rlib -Zpacked_bundled_libs
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "U.*native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep.rust_dep.f8908835-cgu.0.rcgu.o:
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
---------------- T __ZN8rust_dep8rust_dep17h46da1ffdf057d344E
                 U _native_f1

[[[ end stdout ]]]
ar t /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" "native_dep.ext"
[[[ begin stdout ]]]
__.SYMDEF
lib.rmeta
rust_dep.rust_dep.f8908835-cgu.0.rcgu.o
native_dep.ext

[[[ end stdout ]]]
# Make sure compiler doesn't use files, that it shouldn't know about.
rm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/native_dep.ext
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  main.rs --extern rust_dep=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib -Zpacked_bundled_libs
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
warning: 1 warning emitted

/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib:lib.rmeta: no symbols
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-arch" "x86_64" "-m64" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/rustcUWdRqn/symbols.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.0.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.1.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.2.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.3.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.4.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.4g86d6gt786od2ny.rcgu.o" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/rustcUWdRqn/native_dep.ext" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-b706291094b027ef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-26793481f52a6183.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libobject-255e039944a74112.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libmemchr-1b5390a4c4ea9670.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libaddr2line-7ff5ded3cc87cac1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libgimli-3bc94adda00ae909.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-3b1acde86cef02df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd_detect-475d6817bf154f91.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-3f74afb4c8b5ee8d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libminiz_oxide-f41ecafa3ea6ace0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libadler-3f9d4598433d8696.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-f17787f13d366dbc.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-844b6239e11df154.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-31edd1de4ccf8e64.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-626e8a15f1ab42d5.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-1a1a8a9181840fa7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-7763b67623a38e07.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-3775c6f0a76e9fa6.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-357b73ae6c57f814.rlib" "-lSystem" "-lc" "-lm" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main" "-Wl,-dead_strip" "-nodefaultlibs"
  = note: ld: in /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib(native_dep.ext), archive member 'native_dep.ext' with length 8016176 is not mach-o or llvm bitcode file '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib'
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error

make: *** [all] Error 1
---
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -v -c -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o native_dep_1.c
ar crus /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.a /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o
/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -v -c -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o native_dep_2.c
ar crus /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.a /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o
/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -v -c -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o native_dep_3.c
ar crus /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.a /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  rust_dep_up.rs --crate-type=rlib -Zpacked_bundled_libs
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "U.*native_f2"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
                 U _native_f2
                 U _native_f3

[[[ end stdout ]]]
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "U.*native_f3"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
                 U _native_f2
                 U _native_f3

[[[ end stdout ]]]
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "T.*rust_dep_up"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
                 U _native_f2
                 U _native_f3

[[[ end stdout ]]]
ar t /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" "native_dep_2"
[[[ begin stdout ]]]
__.SYMDEF
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
libnative_dep_2.a
libnative_dep_3.a

[[[ end stdout ]]]
ar t /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" "native_dep_3"
[[[ begin stdout ]]]
__.SYMDEF
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
libnative_dep_2.a
libnative_dep_3.a

[[[ end stdout ]]]
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  rust_dep_local.rs --extern rlib=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib -Zpacked_bundled_libs --crate-type=rlib
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "U.*native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-apple-darwin target=x86_64-apple-darwin
---------------- T __ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
                 U _native_f1

[[[ end stdout ]]]
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "T.*rust_dep_local"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U __ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
---------------- T __ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U __ZN4core9panicking5panic17hc4e5b368fe6d3dc3E
                 U _native_f1

[[[ end stdout ]]]
ar t /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" "native_dep_1"
[[[ begin stdout ]]]
__.SYMDEF
lib.rmeta
rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o
libnative_dep_1.a

[[[ end stdout ]]]
# Make sure compiler doesn't use files, that it shouldn't know about.
rm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/*native_dep_*
DYLD_LIBRARY_PATH="/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  main.rs --extern lib=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.exe -Zpacked_bundled_libs --print link-args | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "native_dep_1.*native_dep_2.*native_dep_3"
[[[ begin stdout ]]]
"cc" "-arch" "x86_64" "-m64" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/symbols.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.0.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.1.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.2.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.3.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.4.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.4g86d6gt786od2ny.rcgu.o" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_1.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_2.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_3.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-b706291094b027ef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-26793481f52a6183.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libobject-255e039944a74112.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libmemchr-1b5390a4c4ea9670.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libaddr2line-7ff5ded3cc87cac1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libgimli-3bc94adda00ae909.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-3b1acde86cef02df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd_detect-475d6817bf154f91.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-3f74afb4c8b5ee8d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libminiz_oxide-f41ecafa3ea6ace0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libadler-3f9d4598433d8696.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-f17787f13d366dbc.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-844b6239e11df154.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-31edd1de4ccf8e64.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-626e8a15f1ab42d5.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-1a1a8a9181840fa7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-7763b67623a38e07.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-3775c6f0a76e9fa6.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-357b73ae6c57f814.rlib" "-lSystem" "-lc" "-lm" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.exe" "-Wl,-dead_strip" "-nodefaultlibs"

[[[ end stdout ]]]
"/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin"/llvm-nm /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.exe | "/Users/runner/work/rust/rust/src/etc/cat-and-grep.sh" -e "T.*native_f1"
[[[ begin stdout ]]]

[[[ end stdout ]]]
Error: cannot match: T.*native_f1
--- stderr -------------------------------
clang version 14.0.5 (https://github.com/tru/llvm-release-build 686807a176470032c208f27da2cc31b1c10777c6)
Target: x86_64-apple-darwin
Thread model: posix
Thread model: posix
InstalledDir: /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin
 (in-process)
 "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang-14" -cc1 -triple x86_64-apple-macosx10.8.0 -Wundef-prefix=TARGET_OS_ -Werror=undef-prefix -Wdeprecated-objc-isa-usage -Werror=deprecated-objc-isa-usage -emit-obj -mrelax-all --mrelax-relocations -disable-free -clear-ast-before-backend -disable-llvm-verifier -discard-value-names -main-file-name native_dep_1.c -mrelocation-model pic -pic-level 2 -mframe-pointer=all -ffp-contract=on -fno-rounding-math -funwind-tables=2 -faligned-alloc-unavailable -target-sdk-version=12.3 -fcompatibility-qualified-id-block-type-checking -fvisibility-inlines-hidden-static-local-var -target-cpu core2 -tune-cpu generic -mllvm -treat-scalable-fixed-error-as-warning -debugger-tuning=lldb -target-linker-version 14.0.5 -v -ffunction-sections -fdata-sections -fcoverage-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -resource-dir /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5 -isysroot /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -internal-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include -internal-isystem /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5/include -internal-externc-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include -fdebug-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -ferror-limit 19 -stack-protector 1 -fblocks -fencode-extended-block-signature -fregister-global-dtors-with-atexit -fgnuc-version=4.2.1 -fmax-type-align=16 -D__GCC_HAVE_DWARF2_CFI_ASM=1 -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o -x c native_dep_1.c
clang -cc1 version 14.0.5 based upon LLVM 14.0.5 default target x86_64-apple-darwin21.6.0
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include"
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/Library/Frameworks"
#include "..." search starts here:
#include <...> search starts here:
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/System/Library/Frameworks (framework directory)
End of search list.
clang version 14.0.5 (https://github.com/tru/llvm-release-build 686807a176470032c208f27da2cc31b1c10777c6)
Thread model: posix
InstalledDir: /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin
 (in-process)
 (in-process)
 "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang-14" -cc1 -triple x86_64-apple-macosx10.8.0 -Wundef-prefix=TARGET_OS_ -Werror=undef-prefix -Wdeprecated-objc-isa-usage -Werror=deprecated-objc-isa-usage -emit-obj -mrelax-all --mrelax-relocations -disable-free -clear-ast-before-backend -disable-llvm-verifier -discard-value-names -main-file-name native_dep_2.c -mrelocation-model pic -pic-level 2 -mframe-pointer=all -ffp-contract=on -fno-rounding-math -funwind-tables=2 -faligned-alloc-unavailable -target-sdk-version=12.3 -fcompatibility-qualified-id-block-type-checking -fvisibility-inlines-hidden-static-local-var -target-cpu core2 -tune-cpu generic -mllvm -treat-scalable-fixed-error-as-warning -debugger-tuning=lldb -target-linker-version 14.0.5 -v -ffunction-sections -fdata-sections -fcoverage-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -resource-dir /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5 -isysroot /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -internal-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include -internal-isystem /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5/include -internal-externc-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include -fdebug-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -ferror-limit 19 -stack-protector 1 -fblocks -fencode-extended-block-signature -fregister-global-dtors-with-atexit -fgnuc-version=4.2.1 -fmax-type-align=16 -D__GCC_HAVE_DWARF2_CFI_ASM=1 -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.o -x c native_dep_2.c
clang -cc1 version 14.0.5 based upon LLVM 14.0.5 default target x86_64-apple-darwin21.6.0
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include"
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/Library/Frameworks"
#include "..." search starts here:
#include <...> search starts here:
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/System/Library/Frameworks (framework directory)
End of search list.
clang version 14.0.5 (https://github.com/tru/llvm-release-build 686807a176470032c208f27da2cc31b1c10777c6)
Thread model: posix
InstalledDir: /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin
 (in-process)
 (in-process)
 "/Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/bin/clang-14" -cc1 -triple x86_64-apple-macosx10.8.0 -Wundef-prefix=TARGET_OS_ -Werror=undef-prefix -Wdeprecated-objc-isa-usage -Werror=deprecated-objc-isa-usage -emit-obj -mrelax-all --mrelax-relocations -disable-free -clear-ast-before-backend -disable-llvm-verifier -discard-value-names -main-file-name native_dep_3.c -mrelocation-model pic -pic-level 2 -mframe-pointer=all -ffp-contract=on -fno-rounding-math -funwind-tables=2 -faligned-alloc-unavailable -target-sdk-version=12.3 -fcompatibility-qualified-id-block-type-checking -fvisibility-inlines-hidden-static-local-var -target-cpu core2 -tune-cpu generic -mllvm -treat-scalable-fixed-error-as-warning -debugger-tuning=lldb -target-linker-version 14.0.5 -v -ffunction-sections -fdata-sections -fcoverage-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -resource-dir /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5 -isysroot /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -internal-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include -internal-isystem /Users/runner/work/rust/rust/clang+llvm-14.0.5-x86_64-apple-darwin/lib/clang/14.0.5/include -internal-externc-isystem /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include -fdebug-compilation-dir=/Users/runner/work/rust/rust/src/test/run-make/rlib-format-packed-bundled-libs -ferror-limit 19 -stack-protector 1 -fblocks -fencode-extended-block-signature -fregister-global-dtors-with-atexit -fgnuc-version=4.2.1 -fmax-type-align=16 -D__GCC_HAVE_DWARF2_CFI_ASM=1 -o /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.o -x c native_dep_3.c
clang -cc1 version 14.0.5 based upon LLVM 14.0.5 default target x86_64-apple-darwin21.6.0
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/local/include"
ignoring nonexistent directory "/Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/Library/Frameworks"
#include "..." search starts here:
#include <...> search starts here:
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/usr/include
 /Applications/Xcode_14.0.1.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk/System/Library/Frameworks (framework directory)
End of search list.
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
warning: ignoring --out-dir flag due to -o flag
error: linking with `cc` failed: exit status: 1
  |
  |
  = note: "cc" "-arch" "x86_64" "-m64" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/symbols.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.0.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.1.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.2.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.3.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.4.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.4g86d6gt786od2ny.rcgu.o" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_1.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_2.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcqHRrV2/libnative_dep_3.a" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-b706291094b027ef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-26793481f52a6183.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libobject-255e039944a74112.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libmemchr-1b5390a4c4ea9670.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libaddr2line-7ff5ded3cc87cac1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libgimli-3bc94adda00ae909.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-3b1acde86cef02df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd_detect-475d6817bf154f91.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-3f74afb4c8b5ee8d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libminiz_oxide-f41ecafa3ea6ace0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libadler-3f9d4598433d8696.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-f17787f13d366dbc.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-844b6239e11df154.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-31edd1de4ccf8e64.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-626e8a15f1ab42d5.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-1a1a8a9181840fa7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-7763b67623a38e07.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-3775c6f0a76e9fa6.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-357b73ae6c57f814.rlib" "-lSystem" "-lc" "-lm" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.exe" "-Wl,-dead_strip" "-nodefaultlibs"
  = note: ld: in /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib(libnative_dep_2.a), archive member 'libnative_dep_2.a' with length 824 is not mach-o or llvm bitcode file '/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib'
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error; 1 warning emitted


/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/llvm-nm: error: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.exe: No such file or directory
make: *** [all] Error 1



failures:
