
2019-07-03T17:22:14.8438802Z normalized stderr:
2019-07-03T17:22:15.0846046Z error[E0080]: Miri evaluation error: type validation failed: encountered 65543, but expected a pointer
2019-07-03T17:22:15.0936029Z   --> D:/a/1/s/src/libstd/sys/windows/compat.rs:68:13
2019-07-03T17:22:15.0936200Z    |
2019-07-03T17:22:15.0936318Z 68 |             mem::transmute::<usize, F>(addr)($($argname),*)
2019-07-03T17:22:15.0936484Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Miri evaluation error: type validation failed: encountered 65543, but expected a pointer
2019-07-03T17:22:15.0936579Z    |
2019-07-03T17:22:15.0936705Z    = note: inside call to `std::sys::windows::c::SetThreadStackGuarantee` at D:/a/1/s/src/libstd/sys/windows/stack_overflow.rs:12:12
2019-07-03T17:22:15.0937192Z    = note: inside call to `std::sys::windows::stack_overflow::Handler::new` at D:/a/1/s/src/libstd/sys/windows/stack_overflow.rs:39:14
2019-07-03T17:22:15.0937351Z    = note: inside call to `std::sys::windows::stack_overflow::init` at D:/a/1/s/src/libstd/rt.rs:34:9
2019-07-03T17:22:15.0937457Z    = note: inside call to `std::rt::lang_start_internal` at D:/a/1/s/src/libstd/rt.rs:64:5
2019-07-03T17:22:15.0937584Z    = note: inside call to `std::rt::lang_start::<()>`
2019-07-03T17:22:15.0937824Z 
2019-07-03T17:22:15.0937917Z error: aborting due to previous error
2019-07-03T17:22:15.0937963Z 
2019-07-03T17:22:15.0938062Z For more information about this error, try `rustc --explain E0080`.
2019-07-03T17:22:15.0938125Z 
2019-07-03T17:22:15.0938158Z 
2019-07-03T17:22:15.0938248Z expected stderr:
2019-07-03T17:22:15.0938290Z 
2019-07-03T17:22:15.0938324Z 
2019-07-03T17:22:15.0938402Z diff of stderr:
2019-07-03T17:22:15.0938445Z 
2019-07-03T17:22:15.0938528Z +error[E0080]: Miri evaluation error: type validation failed: encountered 65543, but expected a pointer
2019-07-03T17:22:15.0938660Z +  --> D:/a/1/s/src/libstd/sys/windows/compat.rs:68:13
2019-07-03T17:22:15.0938764Z +   |
2019-07-03T17:22:15.0938835Z +68 |             mem::transmute::<usize, F>(addr)($($argname),*)
2019-07-03T17:22:15.0938977Z +   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Miri evaluation error: type validation failed: encountered 65543, but expected a pointer
2019-07-03T17:22:15.0939067Z +   |
2019-07-03T17:22:15.0939195Z +   = note: inside call to `std::sys::windows::c::SetThreadStackGuarantee` at D:/a/1/s/src/libstd/sys/windows/stack_overflow.rs:12:12
2019-07-03T17:22:15.0939426Z +   = note: inside call to `std::sys::windows::stack_overflow::Handler::new` at D:/a/1/s/src/libstd/sys/windows/stack_overflow.rs:39:14
2019-07-03T17:22:15.0939573Z +   = note: inside call to `std::sys::windows::stack_overflow::init` at D:/a/1/s/src/libstd/rt.rs:34:9
2019-07-03T17:22:15.0939677Z +   = note: inside call to `std::rt::lang_start_internal` at D:/a/1/s/src/libstd/rt.rs:64:5
2019-07-03T17:22:15.0939846Z +   = note: inside call to `std::rt::lang_start::<()>`
2019-07-03T17:22:15.0939916Z +
2019-07-03T17:22:15.0940003Z +error: aborting due to previous error
2019-07-03T17:22:15.0940069Z +
2019-07-03T17:22:15.0940177Z +For more information about this error, try `rustc --explain E0080`.
2019-07-03T17:22:15.0940250Z +
