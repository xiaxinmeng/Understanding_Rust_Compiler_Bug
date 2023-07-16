
#0  0x00007fb729bb7355 in raise () from /usr/lib/libc.so.6
#1  0x00007fb729ba0853 in abort () from /usr/lib/libc.so.6
#2  0x00007fb729e7d8b7 in std::sys::unix::abort_internal () at library/std/src/sys/unix/mod.rs:167
#3  0x00007fb729e6a7d5 in std::sys_common::util::abort () at library/std/src/sys_common/util.rs:19
#4  0x00007fb729e7cba5 in std::sys::unix::stack_overflow::imp::signal_handler () at library/std/src/sys/unix/stack_overflow.rs:106
#5  <signal handler called>
#6  0x00007fb71daa1674 in core::cmp::min_by (v1=<error reading variable: Cannot access memory at address 0x7fb723f7ffc8>, v2=<error reading variable: Cannot access memory at address 0x7fb723f7ffd0>, compare=0x0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:942
#7  0x00007fb71daa1624 in core::cmp::Ord::min (self=3, other=23) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:617
#8  0x00007fb71daa1654 in core::cmp::min (v1=3, v2=23) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs:922
#9  0x00007fb71d868a5f in <u8 as core::slice::SliceOrd>::compare (left=..., right=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:6690
#10 0x00007fb71d86da6d in core::slice::<impl core::cmp::Ord for [T]>::cmp (self=..., other=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:6512
#11 0x00007fb71d52f9a8 in core::str::traits::<impl core::cmp::Ord for str>::cmp (self=..., other=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:1736
#12 0x00007fb71d4b1019 in alloc::collections::btree::search::search_linear (node=0x7fb723f80420, key=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/collections/btree/search.rs:76
#13 0x00007fb71d4a9bb8 in alloc::collections::btree::search::search_node (node=..., key=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/collections/btree/search.rs:51
#14 0x00007fb71d4aec4b in alloc::collections::btree::search::search_tree (node=..., key=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/collections/btree/search.rs:26
#15 0x00007fb71d574e09 in alloc::collections::btree::map::BTreeMap<K,V>::get (self=0x7fb72476a7e8, key=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/collections/btree/map.rs:569
#16 0x00007fb71d4c8f9c in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71a48dea0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:45
#17 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71a48dea0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#18 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f80858, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#19 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f80858, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#20 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbe318, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#21 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1ea0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#22 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1ea0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#23 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f80a68, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#24 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f80a68, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#25 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#26 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#27 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#28 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f80c78, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#29 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f80c78, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#30 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#31 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#32 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#33 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f80e88, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#34 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f80e88, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#35 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
--Type <RET> for more, q to quit, c to continue without paging--
#36 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#37 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#38 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f81098, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#39 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f81098, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#40 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#41 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#42 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#43 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f812a8, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#44 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f812a8, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#45 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#46 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#47 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#48 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f814b8, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#49 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f814b8, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#50 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#51 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#52 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#53 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f816c8, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#54 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f816c8, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#55 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#56 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:54
#57 0x00007fb71d498410 in core::iter::traits::iterator::Iterator::any::check::{{closure}} (x=0x7fb71f3a1bd0)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2144
#58 0x00007fb71d4d08dd in core::iter::traits::iterator::Iterator::try_fold (self=0x7fb723f818d8, init=(), f=...)
    at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1869
#59 0x00007fb71d4d05f3 in core::iter::traits::iterator::Iterator::any (self=0x7fb723f818d8, f=...) at /home/aaron/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:2148
#60 0x00007fb71d4c8ee0 in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection (self=0x7fb716dbf1c0, context=0x7fb724768360, type_name=...)
    at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/graphql_client_codegen-0.9.0/src/inputs.rs:38
#61 0x00007fb71d4c901a in graphql_client_codegen::inputs::GqlInput::contains_type_without_indirection::{{closure}} (field=0x7fb71f3a1bd0)
