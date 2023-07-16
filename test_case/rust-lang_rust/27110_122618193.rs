

rustc: x86_64-apple-darwin/stage2/test/coretesttest-x86_64-apple-darwin
../src/libcoretest/num/mod.rs:121:47: 121:60 error: `ParseIntError` does not name a structure [E0422]
../src/libcoretest/num/mod.rs:121         assert_eq!("--129".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                ^~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:121:9: 121:99 note: expansion site
../src/libcoretest/num/mod.rs:121:68: 121:94 error: failed to resolve. Use of undeclared type or module `IntErrorKind` [E0433]
../src/libcoretest/num/mod.rs:121         assert_eq!("--129".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:121:9: 121:99 note: expansion site
../src/libcoretest/num/mod.rs:121:68: 121:94 error: unresolved name `IntErrorKind::InvalidDigit` [E0425]
../src/libcoretest/num/mod.rs:121         assert_eq!("--129".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:121:9: 121:99 note: expansion site
../src/libcoretest/num/mod.rs:122:47: 122:60 error: `ParseIntError` does not name a structure [E0422]
../src/libcoretest/num/mod.rs:122         assert_eq!("Съешь".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                ^~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:122:9: 122:99 note: expansion site
../src/libcoretest/num/mod.rs:122:68: 122:94 error: failed to resolve. Use of undeclared type or module `IntErrorKind` [E0433]
../src/libcoretest/num/mod.rs:122         assert_eq!("Съешь".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:122:9: 122:99 note: expansion site
../src/libcoretest/num/mod.rs:122:68: 122:94 error: unresolved name `IntErrorKind::InvalidDigit` [E0425]
../src/libcoretest/num/mod.rs:122         assert_eq!("Съешь".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::InvalidDigit }));
                                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:122:9: 122:99 note: expansion site
../src/libcoretest/num/mod.rs:127:43: 127:56 error: `ParseIntError` does not name a structure [E0422]
../src/libcoretest/num/mod.rs:127         assert_eq!("-".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                            ^~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:127:9: 127:88 note: expansion site
../src/libcoretest/num/mod.rs:127:64: 127:83 error: failed to resolve. Use of undeclared type or module `IntErrorKind` [E0433]
../src/libcoretest/num/mod.rs:127         assert_eq!("-".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                                                 ^~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:127:9: 127:88 note: expansion site
../src/libcoretest/num/mod.rs:127:64: 127:83 error: unresolved name `IntErrorKind::Empty` [E0425]
../src/libcoretest/num/mod.rs:127         assert_eq!("-".parse::<i8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                                                 ^~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:127:9: 127:88 note: expansion site
../src/libcoretest/num/mod.rs:128:42: 128:55 error: `ParseIntError` does not name a structure [E0422]
../src/libcoretest/num/mod.rs:128         assert_eq!("".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                           ^~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:128:9: 128:87 note: expansion site
../src/libcoretest/num/mod.rs:128:63: 128:82 error: failed to resolve. Use of undeclared type or module `IntErrorKind` [E0433]
../src/libcoretest/num/mod.rs:128         assert_eq!("".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                                                ^~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:128:9: 128:87 note: expansion site
../src/libcoretest/num/mod.rs:128:63: 128:82 error: unresolved name `IntErrorKind::Empty` [E0425]
../src/libcoretest/num/mod.rs:128         assert_eq!("".parse::<u8>(), Err(ParseIntError{ kind: IntErrorKind::Empty }));
                                                                                                ^~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
../src/libcoretest/num/mod.rs:128:9: 128:87 note: expansion site
error: aborting due to 12 previous errors
make: *** [x86_64-apple-darwin/stage2/test/coretesttest-x86_64-apple-darwin] Error 101
