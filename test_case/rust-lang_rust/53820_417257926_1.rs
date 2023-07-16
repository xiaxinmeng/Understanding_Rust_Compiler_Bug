
#16319 0x00007ffff6dcc218 in rustc_mir::hair::pattern::_match::is_useful () at librustc_mir/hair/pattern/_match.rs:1111
#16320 0x00007ffff6dccf32 in rustc_mir::hair::pattern::_match::is_useful_specialized () at librustc_mir/hair/pattern/_match.rs:1212
#16321 0x00007ffff6af5cd3 in rustc_mir::hair::pattern::_match::is_useful::{{closure}} () at librustc_mir/hair/pattern/_match.rs:1101
#16322 <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold::{{closure}} () at /home/r/src/rust/rustc.2/src/libcore/iter/mod.rs:1406
#16323 core::iter::iterator::Iterator::try_fold () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1522
#16324 <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold () at /home/r/src/rust/rustc.2/src/libcore/iter/mod.rs:1406
#16325 0x00007ffff6b0eb29 in <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold ()
   from /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_mir-336e26fd9f56cf35.so
#16326 0x00007ffff6dca95b in core::iter::iterator::Iterator::try_for_each () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1559
#16327 core::iter::iterator::Iterator::find () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1782
#16328 rustc_mir::hair::pattern::_match::is_useful () at librustc_mir/hair/pattern/_match.rs:1052
#16329 0x00007ffff6dccf32 in rustc_mir::hair::pattern::_match::is_useful_specialized () at librustc_mir/hair/pattern/_match.rs:1212
#16330 0x00007ffff6af5cd3 in rustc_mir::hair::pattern::_match::is_useful::{{closure}} () at librustc_mir/hair/pattern/_match.rs:1101
#16331 <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold::{{closure}} () at /home/r/src/rust/rustc.2/src/libcore/iter/mod.rs:1406
#16332 core::iter::iterator::Iterator::try_fold () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1522
#16333 <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold () at /home/r/src/rust/rustc.2/src/libcore/iter/mod.rs:1406
#16334 0x00007ffff6b0eb29 in <core::iter::Map<I, F> as core::iter::iterator::Iterator>::try_fold ()
   from /home/r/src/rust/rustc.2/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/../lib/librustc_mir-336e26fd9f56cf35.so
#16335 0x00007ffff6dca95b in core::iter::iterator::Iterator::try_for_each () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1559
#16336 core::iter::iterator::Iterator::find () at /home/r/src/rust/rustc.2/src/libcore/iter/iterator.rs:1782
#16337 rustc_mir::hair::pattern::_match::is_useful () at librustc_mir/hair/pattern/_match.rs:1052
#16338 0x00007ffff6dd00fe in rustc_mir::hair::pattern::check_match::check_arms () at librustc_mir/hair/pattern/check_match.rs:369
#16339 rustc_mir::hair::pattern::check_match::MatchVisitor::check_match::{{closure}} () at librustc_mir/hair/pattern/check_match.rs:220
#16340 rustc_mir::hair::pattern::_match::MatchCheckCtxt::create_and_enter () at librustc_mir/hair/pattern/_match.rs:325
