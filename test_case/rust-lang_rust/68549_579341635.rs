
$ ./miri run-debug align.rs -Zmiri-track-pointer-tag=1287
note: popped tracked tag for item [Unique for <1287>]
    --> /home/r/.rustup/toolchains/miri/lib/rustlib/src/rust/src/libcore/slice/mod.rs:2515:48
     |
2515 |                 from_raw_parts_mut(mut_ptr.add(rest.len() - ts_len), ts_len),
     |                                                ^^^^^^^^^^ popped tracked tag for item [Unique for <1287>]
     |
note: inside call to `core::slice::<impl [u8]>::align_to_mut::<[u8; 2]>` at align.rs:3:10
    --> align.rs:3:10
     |
3    |         ([1u8,2,3,4,5].align_to_mut::<[u8;2]>().1)[0]
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
