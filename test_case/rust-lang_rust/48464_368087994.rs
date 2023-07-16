

#0  0x00007fc8f9e14f70 in raise () from /lib64/libc.so.6
#1  0x00007fc8f9e16b3d in abort () from /lib64/libc.so.6
#2  0x000055d97fc994b9 in panic_abort::__rust_start_panic::abort () at libpanic_abort/lib.rs:62
#3  0x000055d97fc994a9 in __rust_start_panic () at libpanic_abort/lib.rs:57
#4  0x000055d97fc82cba in rust_panic () at libstd/panicking.rs:431
#5  std::panicking::rust_panic_with_hook () at libstd/panicking.rs:416
#6  0x000055d97fc8254e in std::panicking::begin_panic_fmt () at libstd/panicking.rs:347
#7  0x000055d97fc9b40b in rust_begin_unwind () at libstd/panicking.rs:323
#8  core::panicking::panic_fmt () at libcore/panicking.rs:71
#9  0x000055d97fca1e59 in core::option::expect_failed () at libcore/option.rs:917
#10 0x000055d97fb182f4 in <std::collections::hash::table::RawTable<K, V>>::new (capacity=576460752303423488) at /checkout/src/libcore/option.rs:302
#11 0x000055d97fb0991d in <std::collections::hash::map::HashMap<K, V, S>>::resize (self=0x7ffcd0d78c20, new_raw_cap=140723812268032)
    at /checkout/src/libstd/collections/hash/map.rs:802
#12 0x000055d97fb0969d in <std::collections::hash::map::HashMap<K, V, S>>::entry (self=0x7ffcd0d78c20, key=...) at /checkout/src/libcore/cmp.rs:462
#13 0x000055d97fb30460 in alacritty::display::Display::new (config=0x7ffcd0d79f10, options=0x7ffcd0d7a5e8) at src/renderer/mod.rs:282
#14 0x000055d97faad14a in alacritty::main () at src/main.rs:104
