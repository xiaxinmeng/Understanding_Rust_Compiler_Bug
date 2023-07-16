
#6  <signal handler called>
#7  0x000055ef6590d911 in core::array::try_from_fn<core::ops::try_trait::NeverShortCircuit<alloc::vec::Vec<u32, alloc::alloc::Global>>, 50000, core::array::try_from_trusted_iterator::next::{closure_env#0}<core::ops::try_trait::NeverShortCircuit<alloc::vec::Vec<u32, alloc::alloc::Global>>, core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<alloc::vec::Vec<u32, alloc::alloc::Global>>>, fn(alloc::vec::Vec<u32, alloc::alloc::Global>) -> core::ops::try_trait::NeverShortCircuit<alloc::vec::Vec<u32, alloc::alloc::Global>>>>> (cb=...)
    at /home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:97
#8  0x000055ef6590de57 in core::array::try_from_trusted_iterator<alloc::vec::Vec<u32, alloc::alloc::Global>, core::ops::try_trait::NeverShortCircuit<alloc::vec::Vec<u32, alloc::alloc::Global>>, 50000, core::iter::adapters::map::Map<core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<alloc::vec::Vec<u32, alloc::alloc::Global>>>, fn(alloc::vec::Vec<u32, alloc::alloc::Global>) -> core::ops::try_trait::NeverShortCircuit<alloc::vec::Vec<u32, alloc::alloc::Global>>>> (iter=...)
    at /home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:815
#9  0x000055ef6590e677 in core::array::from_trusted_iterator<alloc::vec::Vec<u32, alloc::alloc::Global>, 50000, core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<alloc::vec::Vec<u32, alloc::alloc::Global>>>> (iter=...)
    at /home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:795
#10 core::array::{impl#21}::clone<alloc::vec::Vec<u32, alloc::alloc::Global>, 50000> (array=0x7fff38cc3330)
    at /home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:416
#11 core::array::{impl#20}::clone<alloc::vec::Vec<u32, alloc::alloc::Global>, 50000> (self=0x7fff38cc3330)
    at /home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/array/mod.rs:400
#12 scratch::main () at src/main.rs:6
