
test> rustc -g nested.rs --target=i686-pc-windows-msvc ; ./nested.exe
thread 'main' panicked at 'Nesting!', nested.rs:8
stack backtrace:
   0:   0xfa788b - std::rt::lang_start::haaae1186de9de8cb
   1:   0xfa810a - std::panicking::rust_panic_with_hook::hb1322e5f2588b4db
   2:   0xfa10ec - std::panicking::begin_panic<&str>
                at C:\bot\slave\nightly-dist-rustc-win-msvc-32\build\src\libstd\panicking.rs:383
   3:   0xfa12df - nested::a::b::c::d::foo
                at C:\msys64\home\Peter\test\nested.rs:8
   4:   0xfa12a9 - nested::a::b::c::foo
                at C:\msys64\home\Peter\test\nested.rs:6
   5:   0xfa1299 - nested::a::b::foo
                at C:\msys64\home\Peter\test\nested.rs:4
   6:   0xfa1289 - nested::a::foo
                at C:\msys64\home\Peter\test\nested.rs:2
   7:   0xfa12e9 - nested::main
                at C:\msys64\home\Peter\test\nested.rs:13
   8:   0xfaaa7b - __rust_maybe_catch_panic
   9:   0xfa7300 - std::rt::lang_start::haaae1186de9de8cb
  10:   0xfa1310 - main
  11: 0x75e438f3 - BaseThreadInitThunk
  12: 0x77935de2 - RtlUnicodeStringToInteger
