
$ printf '%s' '#[no_mangle] pub extern "C" fn my() { std::panic::catch_unwind(|| println!("hello")).unwrap(); }' \ 
    | rustc --crate-type=cdylib - && nm --defined-only --extern-only librust_out.so
0000000000007270 T my
