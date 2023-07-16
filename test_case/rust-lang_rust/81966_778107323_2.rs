
❯ cat src/lib.rs
#[no_mangle]
pub extern "C" fn hello_world() -> u32 {
    1+1
}
❯ rustc +stage1 --target aarch64-apple-ios-sim src/lib.rs --crate-type staticlib
❯ otool -l liblib.a | rg 'VERSION|platform|version' | head -2
      cmd LC_BUILD_VERSION
 platform 7
