
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\os.rs:307:41: 307:46 error: type `core::iter::Range<isize>` does not implement any method in scope named `len`
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\os.rs:307     fn len(&self) -> usize { self.range.len() }
                                                                                                               ^~~~~
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\thread_local.rs:247:42: 247:51 error: the trait `core::clone::Clone` is not implemented for the type `unsafe extern "C" fn(*mut u8)` [E0277]
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\thread_local.rs:247                 (*DTORS).iter().cloned().collect()
                                                                                                                          ^~~~~~~~~
error: aborting due to 2 previous errors
/c/bot/slave/auto-win-32-nopt-t/build/mk/target.mk:165: recipe for target 'i686-pc-windows-gnu/stage0/bin/rustlib/i686-pc-windows-gnu/lib/stamp.std' failed
make: *** [i686-pc-windows-gnu/stage0/bin/rustlib/i686-pc-windows-gnu/lib/stamp.std] Error 101
