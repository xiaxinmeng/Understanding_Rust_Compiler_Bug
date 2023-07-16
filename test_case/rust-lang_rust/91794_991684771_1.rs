
 Documenting alloc v0.0.0 (C:\msys64\home\we\rust\library\alloc)
error: `core::cmp::Ord` is both a trait and a macro
   --> library\alloc\src\collections\binary_heap.rs:254:14
    |
254 | /// [`Ord`]: core::cmp::Ord
    |              ^^^^^^^^^^^^^^ ambiguous link
    |
    = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
help: to link to the trait, prefix with `trait@`
    |
254 | /// [`Ord`]: trait@core::cmp::Ord
    |              ++++++
help: to link to the macro, add an exclamation mark
    |
254 | /// [`Ord`]: core::cmp::Ord!
    |                            +

error: `core::cmp::Ord` is both a trait and a macro
  --> library\alloc\src\collections\btree\set.rs:30:14
   |
30 | /// [`Ord`]: core::cmp::Ord
   |              ^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the trait, prefix with `trait@`
   |
30 | /// [`Ord`]: trait@core::cmp::Ord
   |              ++++++
help: to link to the macro, add an exclamation mark
   |
30 | /// [`Ord`]: core::cmp::Ord!
   |                            +

error: `core::cmp::Ord` is both a trait and a macro
    --> library\alloc\src\vec\mod.rs:2802:57
     |
2802 | /// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
     |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous link
     |
help: to link to the trait, prefix with `trait@`
     |
2802 | /// Implements ordering of vectors, [lexicographically](trait@core::cmp::Ord#lexicographical-comparison).
     |                                                         ++++++
help: to link to the macro, add an exclamation mark
     |
2802 | /// Implements ordering of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison!).
     |                                                                                                  +

error: `core::cmp::Ord` is both a trait and a macro
    --> library\alloc\src\vec\mod.rs:2790:59
     |
2790 | /// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison).
     |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous link
     |
help: to link to the trait, prefix with `trait@`
     |
2790 | /// Implements comparison of vectors, [lexicographically](trait@core::cmp::Ord#lexicographical-comparison).
     |                                                           ++++++
help: to link to the macro, add an exclamation mark
     |
2790 | /// Implements comparison of vectors, [lexicographically](core::cmp::Ord#lexicographical-comparison!).
     |                                                                                                    +

error: could not document `alloc`

Caused by:
  process didn't exit successfully: `C:\msys64\home\we\rust\build\bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc 'library\alloc\src\lib.rs' --target x86_64-pc-windows-gnu -o 'C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\doc' --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.59.0 --index-page 'C:\msys64\home\we\rust\src/doc/index.md' -C metadata=a7226ac55a8e3b93 -L 'dependency=C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\release\deps' -L 'dependency=C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage2-std\release\deps' --extern 'compiler_builtins=C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\release\deps\libcompiler_builtins-4ccdf02c8b2f7747.rmeta' --extern 'core=C:\msys64\home\we\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\release\deps\libcore-260e3f3523ff358b.rmeta' -Ctarget-cpu=native -Zsymbol-mangling-version=legacy -Clink-arg=-fuse-ld=lld '-Clink-arg=-Wl,/threads:1' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version 1.59.0-dev '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit code: 1)


command did not execute successfully: "\\\\?\\C:\\msys64\\home\\we\\rust\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "rustdoc" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "C:\\msys64\\home\\we\\rust\\library/test/Cargo.toml" "-p" "alloc" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.59.0" "--index-page" "C:\\msys64\\home\\we\\rust\\src/doc/index.md"
expected success, got: exit code: 101
