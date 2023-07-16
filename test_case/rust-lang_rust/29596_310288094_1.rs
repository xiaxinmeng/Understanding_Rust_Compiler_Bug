
rustc 1.18.0 (03fc9d622 2017-06-06)
warning: unused attribute
 --> <anon>:2:1
  |
2 | #![link_args = "-l bunny"]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_attributes)] on by default

error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "./out.0.o" "-o" "./out" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f4594d3e53dcb114.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-1efbcfd8938372b6.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcollections-532a3dbf317eff86.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-cfbd6648f7db2ee5.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a0157c0ca919c364.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-488b4ab4bd53a138.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-ca07b617414dd0fa.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_jemalloc-492d8ea7fa3384ff.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-88c194c15fdb6521.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-687e6a964d22cbb4.rlib" "/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-987729be881d4d32.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "gcc_s" "-l" "pthread" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "bunny"
  = note: /usr/bin/ld: cannot find -lbunny
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error
