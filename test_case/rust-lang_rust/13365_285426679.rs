
rustc 1.17.0-nightly (b1e31766d 2017-03-03)
warning: static variable `arr` should have an upper case name such as `ARR`
 --> <anon>:4:1
  |
4 | pub static arr: [u8; 16] = [0; 16];
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(non_upper_case_globals)] on by default

error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "./out.0.o" "-o" "./out" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-9a66b6a343d52844.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-6bc49e032a89c77d.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-a2a467c3ca3b6479.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-e54225ff8f33e08f.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-9d79f761aa668a33.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-2beb731af7a6faec.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-ce7b9706e1719f27.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-4b74d6f2808677d3.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-95af4192ed69a1c8.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-cd0ca85e71f914ca.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-0bf24067248742a8.rlib" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "util"
  = note: /usr/bin/ld: ./out.0.o: relocation R_X86_64_32S against `arr' can not be used when making a shared object; recompile with -fPIC
          ./out.0.o: error adding symbols: Bad value
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error
