
$ cat foo.rs
fn test(bla: i32) -> i32 { bla*bla }

#[no_mangle]
pub extern "C" fn _test_wrapper(i: i32) -> i32 {
  test(i)
}
$ rustc --crate-type dylib foo.rs
$ nm -g libfoo.so
00000000000024df A __bss_start
                 w __cxa_finalize
00000000000024df A _edata
00000000000024e4 A _end
00000000000007c8 T _fini
                 w __gmon_start__
00000000000005b0 T _init
                 w _ITM_deregisterTMCloneTable
                 w _ITM_registerTMCloneTable
                 w _Jv_RegisterClasses
0000000000002020 D rust_metadata_foo_17a5f6ae9433831f
                 U rust_stack_exhausted
0000000000000740 T _test_wrapper
$ rustc -v
rustc 0.13.0-nightly (b87619e27 2014-11-02 23:27:10 +0000)
