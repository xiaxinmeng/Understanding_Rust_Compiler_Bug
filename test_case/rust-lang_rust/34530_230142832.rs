
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\num/f32.rs:114:26: 114:36 error: use of deprecated item: never really came to fruition and easily implementable outside the standard library
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\num/f32.rs:114             let (a, b) = f64::frexp(x as f64);
                                                                                                 ^~~~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
                                                                                                  ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\num/f32.rs:121:13: 121:23 error: use of deprecated item: never really came to fruition and easily implementable outside the standard library
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\num/f32.rs:121             f64::ldexp(x as f64, n as isize) as c_float
                                                                                    ^~~~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
                                                                                                  ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:17:5: 17:14 error: use of deprecated item: no longer used for Iterator::sum
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:17 use num::Zero;
                                                                                   ^~~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
                                                                                                  ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows\net.rs:20:5: 20:13 error: unused import
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows\net.rs:20 use ops::Neg;
                                                                                   ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
                                                                                                  ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:181:23: 181:27 error: use of deprecated item: no longer used for Iterator::sum
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:181 fn cvt<I: PartialEq + Zero>(i: I) -> io::Result<I> {
                                                                                                      ^~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
                                                                                                  ^~~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:182:13: 182:20 error: use of deprecated item: no longer used for Iterator::sum
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\sys/windows/mod.rs:182     if i == I::zero() {
                                                                                            ^~~~~~~
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289:31: 289:39 note: lint level defined here
C:\bot\slave\auto-win-msvc-64-cargotest\build\src\libstd\lib.rs:289 #![cfg_attr(not(stage0), deny(warnings))]
