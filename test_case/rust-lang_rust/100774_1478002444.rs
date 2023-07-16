shell
$ printf '%s' '#[no_mangle] pub extern "C" fn my() { std::panic::catch_unwind(|| println!("hello")).unwrap(); }' \
    | rustc --crate-type=cdylib - && dyld_info -exports librust_out.dylib
librust_out.dylib [arm64]:
    -exports:
        offset      symbol
        0x000048E0  _my
