
$ touch foo.rs
$ rustc +nightly foo.rs --crate-type cdylib
$ nm -g libfoo.so | grep -w T
0000000000008590 T __rdl_alloc
00000000000087a0 T __rdl_alloc_excess
0000000000008710 T __rdl_alloc_zeroed
0000000000008630 T __rdl_dealloc
00000000000088f0 T __rdl_grow_in_place
0000000000008600 T __rdl_oom
0000000000008650 T __rdl_realloc
0000000000008820 T __rdl_realloc_excess
0000000000008900 T __rdl_shrink_in_place
0000000000008640 T __rdl_usable_size
0000000000012db0 T rust_eh_personality
