
$ nm -g ../tiny-live-code-example/target/debug/libstate_manipulation1000.so | grep -w T
0000000000003530 T lib_update_and_render
000000000000a110 T __rdl_alloc
000000000000a300 T __rdl_alloc_excess
000000000000a280 T __rdl_alloc_zeroed
000000000000a1b0 T __rdl_dealloc
000000000000a430 T __rdl_grow_in_place
000000000000a180 T __rdl_oom
000000000000a1d0 T __rdl_realloc
000000000000a370 T __rdl_realloc_excess
000000000000a440 T __rdl_shrink_in_place
000000000000a1c0 T __rdl_usable_size
0000000000014a70 T rust_eh_personality
