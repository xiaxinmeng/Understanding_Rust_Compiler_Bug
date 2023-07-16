plain
2019-10-24T05:58:54.8515404Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T05:58:54.8723777Z ##[command]git config gc.auto 0
2019-10-24T05:58:54.8824767Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T05:58:54.8893976Z ##[command]git config --get-all http.proxy
2019-10-24T05:58:55.7256691Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-24T06:31:36.2582535Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.2582765Z      |
2019-10-24T06:31:36.2583101Z 7    | / macro_rules! panic {
2019-10-24T06:31:36.2583423Z 8    | |     () => (
2019-10-24T06:31:36.2583751Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.2584312Z ...    |
2019-10-24T06:31:36.2584312Z ...    |
2019-10-24T06:31:36.2584713Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.2585449Z      | |                                                                                |
2019-10-24T06:31:36.2585859Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.2586259Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.2586557Z 33   | |         }
2019-10-24T06:31:36.2586557Z 33   | |         }
2019-10-24T06:31:36.2586865Z 34   | |     });
2019-10-24T06:31:36.2587379Z 35   | | }
2019-10-24T06:31:36.2587792Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:36.2588004Z ...
2019-10-24T06:31:36.2588298Z 1226 | /     macro_rules! assert {
2019-10-24T06:31:36.2588655Z 1227 | |         ($cond:expr) => ({ /* compiler built-in */ });
2019-10-24T06:31:36.2588987Z 1228 | |         ($cond:expr,) => ({ /* compiler built-in */ });
2019-10-24T06:31:36.2589326Z 1229 | |         ($cond:expr, $($arg:tt)+) => ({ /* compiler built-in */ })
2019-10-24T06:31:36.2589946Z      | |_____- in this expansion of `assert!`
2019-10-24T06:31:36.2590181Z      | 
2019-10-24T06:31:36.2595607Z     ::: src/libcore/num/mod.rs:4960:5
2019-10-24T06:31:36.2595982Z      |
2019-10-24T06:31:36.2595982Z      |
2019-10-24T06:31:36.2596296Z 4960 |       assert!(radix >= 2 && radix <= 36,
2019-10-24T06:31:36.2596800Z      | |_____|
2019-10-24T06:31:36.2597344Z      | |
2019-10-24T06:31:36.2597344Z      | |
2019-10-24T06:31:36.2597724Z 4961 | |            "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
2019-10-24T06:31:36.2598030Z 4962 | |            radix);
2019-10-24T06:31:36.2598603Z      | |__________________|
2019-10-24T06:31:36.2598904Z      | |__________________in this macro invocation
2019-10-24T06:31:36.2599206Z      |                    in this macro invocation
2019-10-24T06:31:36.2599420Z      |
---
2019-10-24T06:31:36.3465966Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.3466233Z     |
2019-10-24T06:31:36.3466562Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.3466862Z 8   | |     () => (
2019-10-24T06:31:36.3467200Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.3467954Z ...   |
2019-10-24T06:31:36.3467954Z ...   |
2019-10-24T06:31:36.3468396Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.3469139Z     | |                                                                                |
2019-10-24T06:31:36.3469534Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.3469925Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.3470240Z 33  | |         }
2019-10-24T06:31:36.3470240Z 33  | |         }
2019-10-24T06:31:36.3470525Z 34  | |     });
2019-10-24T06:31:36.3471438Z 35  | | }
2019-10-24T06:31:36.3471881Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.3472403Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.3472403Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.3472730Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.3473045Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.3473377Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.3473633Z ...   |
2019-10-24T06:31:36.3473969Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.3474265Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.3474583Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.3475160Z ...
2019-10-24T06:31:36.3475440Z 91  | |     });
2019-10-24T06:31:36.3475737Z 92  | | }
2019-10-24T06:31:36.3476063Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.3476063Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.3476270Z ...
2019-10-24T06:31:36.3476585Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.3476950Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.3477887Z 226 | | }
2019-10-24T06:31:36.3478219Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.3478507Z     | 
2019-10-24T06:31:36.3478507Z     | 
2019-10-24T06:31:36.3478797Z    ::: src/libcore/num/flt2dec/mod.rs:633:17
2019-10-24T06:31:36.3479034Z     |
2019-10-24T06:31:36.3479373Z 633 |                   debug_assert_eq!(len, 0);
2019-10-24T06:31:36.3480003Z     |
2019-10-24T06:31:36.3480340Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.3480753Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.3480794Z 
2019-10-24T06:31:36.3480794Z 
2019-10-24T06:31:36.3962213Z error[E0308]: mismatched types
2019-10-24T06:31:36.4010827Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4011505Z     |
2019-10-24T06:31:36.4011843Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4012178Z 8   | |     () => (
2019-10-24T06:31:36.4012520Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4015733Z ...   |
2019-10-24T06:31:36.4015733Z ...   |
2019-10-24T06:31:36.4016113Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4016847Z     | |                                                                                |
2019-10-24T06:31:36.4017278Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4017670Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4018001Z 33  | |         }
2019-10-24T06:31:36.4018001Z 33  | |         }
2019-10-24T06:31:36.4018288Z 34  | |     });
2019-10-24T06:31:36.4018586Z 35  | | }
2019-10-24T06:31:36.4018895Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4019695Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4019695Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4020015Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4020336Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4020670Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4021297Z ...   |
2019-10-24T06:31:36.4021636Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4021947Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4022267Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4023055Z ...
2019-10-24T06:31:36.4023334Z 91  | |     });
2019-10-24T06:31:36.4023628Z 92  | | }
2019-10-24T06:31:36.4023947Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4023947Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4024155Z ...
2019-10-24T06:31:36.4024477Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4024846Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4025578Z 226 | | }
2019-10-24T06:31:36.4025893Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4026148Z     | 
2019-10-24T06:31:36.4026148Z     | 
2019-10-24T06:31:36.4026427Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:188:5
2019-10-24T06:31:36.4026661Z     |
2019-10-24T06:31:36.4026947Z 188 |       debug_assert_eq!(plus.e, minus.e);
2019-10-24T06:31:36.4027536Z     |
2019-10-24T06:31:36.4027824Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4028074Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4028114Z 
2019-10-24T06:31:36.4028114Z 
2019-10-24T06:31:36.4062630Z error[E0308]: mismatched types
2019-10-24T06:31:36.4062975Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4064365Z     |
2019-10-24T06:31:36.4064824Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4065130Z 8   | |     () => (
2019-10-24T06:31:36.4065485Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4066047Z ...   |
2019-10-24T06:31:36.4066047Z ...   |
2019-10-24T06:31:36.4066414Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4067164Z     | |                                                                                |
2019-10-24T06:31:36.4067560Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4068125Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4068421Z 33  | |         }
2019-10-24T06:31:36.4068421Z 33  | |         }
2019-10-24T06:31:36.4068731Z 34  | |     });
2019-10-24T06:31:36.4069015Z 35  | | }
2019-10-24T06:31:36.4069322Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4069856Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4069856Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4070165Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4070489Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4071226Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4071533Z ...   |
2019-10-24T06:31:36.4071902Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4072175Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4072501Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4073072Z ...
2019-10-24T06:31:36.4073364Z 91  | |     });
2019-10-24T06:31:36.4073784Z 92  | | }
2019-10-24T06:31:36.4074150Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4074150Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4074380Z ...
2019-10-24T06:31:36.4074682Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4075058Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4075791Z 226 | | }
2019-10-24T06:31:36.4076125Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4076356Z     | 
2019-10-24T06:31:36.4076356Z     | 
2019-10-24T06:31:36.4076626Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:189:5
2019-10-24T06:31:36.4076992Z     |
2019-10-24T06:31:36.4077273Z 189 |       debug_assert_eq!(plus.e, v.e);
2019-10-24T06:31:36.4077851Z     |
2019-10-24T06:31:36.4078137Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4078412Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4078452Z 
2019-10-24T06:31:36.4078452Z 
2019-10-24T06:31:36.4188315Z error[E0308]: mismatched types
2019-10-24T06:31:36.4188782Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4189017Z     |
2019-10-24T06:31:36.4189348Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4189657Z 8   | |     () => (
2019-10-24T06:31:36.4189977Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4190535Z ...   |
2019-10-24T06:31:36.4190535Z ...   |
2019-10-24T06:31:36.4190904Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4192043Z     | |                                                                                |
2019-10-24T06:31:36.4192447Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4193053Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4193415Z 33  | |         }
2019-10-24T06:31:36.4193415Z 33  | |         }
2019-10-24T06:31:36.4193731Z 34  | |     });
2019-10-24T06:31:36.4194022Z 35  | | }
2019-10-24T06:31:36.4194345Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4194854Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4194854Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4195183Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4195502Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4195821Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4196083Z ...   |
2019-10-24T06:31:36.4196565Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4196844Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4197186Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4197769Z ...
2019-10-24T06:31:36.4198048Z 91  | |     });
2019-10-24T06:31:36.4198326Z 92  | | }
2019-10-24T06:31:36.4198664Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4198664Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4198877Z ...
2019-10-24T06:31:36.4199174Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4199554Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4200291Z 226 | | }
2019-10-24T06:31:36.4200617Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4200843Z     | 
2019-10-24T06:31:36.4200843Z     | 
2019-10-24T06:31:36.4201678Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:268:13
2019-10-24T06:31:36.4201915Z     |
2019-10-24T06:31:36.4202207Z 268 |               debug_assert_eq!(ten_kappa, 1);
2019-10-24T06:31:36.4202995Z     |
2019-10-24T06:31:36.4203309Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4203572Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4203622Z 
2019-10-24T06:31:36.4203622Z 
2019-10-24T06:31:36.4279205Z error[E0308]: mismatched types
2019-10-24T06:31:36.4279640Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4279865Z     |
2019-10-24T06:31:36.4280187Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4280478Z 8   | |     () => (
2019-10-24T06:31:36.4280820Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4281767Z ...   |
2019-10-24T06:31:36.4281767Z ...   |
2019-10-24T06:31:36.4282449Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4283183Z     | |                                                                                |
2019-10-24T06:31:36.4283593Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4284003Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4284315Z 33  | |         }
2019-10-24T06:31:36.4284315Z 33  | |         }
2019-10-24T06:31:36.4284603Z 34  | |     });
2019-10-24T06:31:36.4284901Z 35  | | }
2019-10-24T06:31:36.4285207Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4285745Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4285745Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4286056Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4286384Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4286711Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4286955Z ...   |
2019-10-24T06:31:36.4287311Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4287690Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4288045Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4289454Z ...
2019-10-24T06:31:36.4289743Z 91  | |     });
2019-10-24T06:31:36.4290047Z 92  | | }
2019-10-24T06:31:36.4290362Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4290362Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4290618Z ...
2019-10-24T06:31:36.4290965Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4291684Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4292640Z 226 | | }
2019-10-24T06:31:36.4292981Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4293239Z     | 
2019-10-24T06:31:36.4293239Z     | 
2019-10-24T06:31:36.4293547Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:269:13
2019-10-24T06:31:36.4293810Z     |
2019-10-24T06:31:36.4294127Z 269 |               debug_assert_eq!(kappa, 0);
2019-10-24T06:31:36.4294830Z     |
2019-10-24T06:31:36.4295109Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4295354Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4295408Z 
2019-10-24T06:31:36.4295408Z 
2019-10-24T06:31:36.4538703Z error[E0308]: mismatched types
2019-10-24T06:31:36.4539169Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4539447Z     |
2019-10-24T06:31:36.4539865Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4540162Z 8   | |     () => (
2019-10-24T06:31:36.4540502Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4541394Z ...   |
2019-10-24T06:31:36.4541394Z ...   |
2019-10-24T06:31:36.4541829Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4542834Z     | |                                                                                |
2019-10-24T06:31:36.4543249Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4543670Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4544513Z 33  | |         }
2019-10-24T06:31:36.4544513Z 33  | |         }
2019-10-24T06:31:36.4544852Z 34  | |     });
2019-10-24T06:31:36.4545146Z 35  | | }
2019-10-24T06:31:36.4545456Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4546183Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4546183Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4546499Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4546832Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4547146Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4547414Z ...   |
2019-10-24T06:31:36.4547758Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4548034Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4549164Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4549802Z ...
2019-10-24T06:31:36.4550104Z 91  | |     });
2019-10-24T06:31:36.4550397Z 92  | | }
2019-10-24T06:31:36.4550733Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4550733Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4550942Z ...
2019-10-24T06:31:36.4551571Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4551990Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4552710Z 226 | | }
2019-10-24T06:31:36.4553218Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4553499Z     | 
2019-10-24T06:31:36.4553499Z     | 
2019-10-24T06:31:36.4553797Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:514:13
2019-10-24T06:31:36.4554026Z     |
2019-10-24T06:31:36.4554318Z 514 |               debug_assert_eq!(ten_kappa, 1);
2019-10-24T06:31:36.4554909Z     |
2019-10-24T06:31:36.4555213Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4555481Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4555522Z 
2019-10-24T06:31:36.4555522Z 
2019-10-24T06:31:36.4782111Z error[E0308]: mismatched types
2019-10-24T06:31:36.4782489Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.4782717Z     |
2019-10-24T06:31:36.4783362Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.4783663Z 8   | |     () => (
2019-10-24T06:31:36.4784738Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.4785423Z ...   |
2019-10-24T06:31:36.4785423Z ...   |
2019-10-24T06:31:36.4785819Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.4786562Z     | |                                                                                |
2019-10-24T06:31:36.4786976Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.4787388Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.4787693Z 33  | |         }
2019-10-24T06:31:36.4787693Z 33  | |         }
2019-10-24T06:31:36.4787982Z 34  | |     });
2019-10-24T06:31:36.4788290Z 35  | | }
2019-10-24T06:31:36.4788598Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:36.4789118Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4789118Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.4789430Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.4789943Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.4790316Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.4790562Z ...   |
2019-10-24T06:31:36.4790927Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.4791534Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.4791871Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.4792461Z ...
2019-10-24T06:31:36.4792740Z 91  | |     });
2019-10-24T06:31:36.4793038Z 92  | | }
2019-10-24T06:31:36.4793527Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4793527Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:36.4793756Z ...
2019-10-24T06:31:36.4794061Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:36.4794425Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:36.4795165Z 226 | | }
2019-10-24T06:31:36.4795501Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:36.4795740Z     | 
2019-10-24T06:31:36.4795740Z     | 
2019-10-24T06:31:36.4796012Z    ::: src/libcore/num/flt2dec/strategy/grisu.rs:515:13
2019-10-24T06:31:36.4796246Z     |
2019-10-24T06:31:36.4796533Z 515 |               debug_assert_eq!(kappa, 0);
2019-10-24T06:31:36.4797124Z     |
2019-10-24T06:31:36.4797412Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.4797665Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.4797729Z 
2019-10-24T06:31:36.4797729Z 
2019-10-24T06:31:36.5577395Z error[E0308]: mismatched types
2019-10-24T06:31:36.5577779Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.5578033Z     |
2019-10-24T06:31:36.5578334Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.5578630Z 8   | |     () => (
2019-10-24T06:31:36.5579205Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.5579902Z ...   |
2019-10-24T06:31:36.5579902Z ...   |
2019-10-24T06:31:36.5580276Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.5581191Z     | |                                                                                |
2019-10-24T06:31:36.5581628Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.5582041Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.5582508Z 33  | |         }
2019-10-24T06:31:36.5582508Z 33  | |         }
2019-10-24T06:31:36.5582819Z 34  | |     });
2019-10-24T06:31:36.5583100Z 35  | | }
2019-10-24T06:31:36.5583396Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:36.5583635Z     | 
2019-10-24T06:31:36.5583902Z    ::: src/libcore/num/dec2flt/rawfp.rs:250:9
2019-10-24T06:31:36.5584116Z     |
2019-10-24T06:31:36.5584437Z 250 |           panic!("fp_to_float: exponent {} too large", e)
2019-10-24T06:31:36.5585020Z     |
2019-10-24T06:31:36.5585322Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.5585572Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.5585611Z 
2019-10-24T06:31:36.5585611Z 
2019-10-24T06:31:36.5736323Z error[E0308]: mismatched types
2019-10-24T06:31:36.5736677Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.5736913Z     |
2019-10-24T06:31:36.5737241Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.5737535Z 8   | |     () => (
2019-10-24T06:31:36.5737866Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.5738900Z ...   |
2019-10-24T06:31:36.5738900Z ...   |
2019-10-24T06:31:36.5739299Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.5740393Z     | |                                                                                |
2019-10-24T06:31:36.5740947Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.5741702Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.5742053Z 33  | |         }
2019-10-24T06:31:36.5742053Z 33  | |         }
2019-10-24T06:31:36.5742357Z 34  | |     });
2019-10-24T06:31:36.5742658Z 35  | | }
2019-10-24T06:31:36.5742960Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:36.5743378Z     | 
2019-10-24T06:31:36.5743661Z    ::: src/libcore/num/dec2flt/rawfp.rs:254:9
2019-10-24T06:31:36.5743879Z     |
2019-10-24T06:31:36.5744184Z 254 |           panic!("fp_to_float: exponent {} too small", e)
2019-10-24T06:31:36.5744789Z     |
2019-10-24T06:31:36.5745080Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.5745351Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.5745391Z 
2019-10-24T06:31:36.5745391Z 
2019-10-24T06:31:36.5898000Z error[E0308]: mismatched types
2019-10-24T06:31:36.5898350Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.5898580Z     |
2019-10-24T06:31:36.5898877Z 7   | / macro_rules! panic {
2019-10-24T06:31:36.5899192Z 8   | |     () => (
2019-10-24T06:31:36.5899535Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.5900090Z ...   |
2019-10-24T06:31:36.5900090Z ...   |
2019-10-24T06:31:36.5900443Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.5901659Z     | |                                                                                |
2019-10-24T06:31:36.5902354Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.5902844Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.5903147Z 33  | |         }
2019-10-24T06:31:36.5903147Z 33  | |         }
2019-10-24T06:31:36.5903474Z 34  | |     });
2019-10-24T06:31:36.5903755Z 35  | | }
2019-10-24T06:31:36.5904056Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:36.5904283Z ...
2019-10-24T06:31:36.5904584Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:36.5904899Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.5905232Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:36.5905764Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:36.5906033Z ...   |
2019-10-24T06:31:36.5906372Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.5906647Z 68  |     left: `{:?}`,
2019-10-24T06:31:36.5906991Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.5907542Z ...
2019-10-24T06:31:36.5907849Z 91  | |     });
2019-10-24T06:31:36.5908126Z 92  | | }
2019-10-24T06:31:36.5908430Z     | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:36.5908430Z     | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:36.5908669Z     | 
2019-10-24T06:31:36.5908930Z    ::: src/libcore/num/dec2flt/rawfp.rs:264:5
2019-10-24T06:31:36.5909145Z     |
2019-10-24T06:31:36.5909463Z 264 |       assert_eq!(q << excess | rem, x.f);
2019-10-24T06:31:36.5910040Z     |
2019-10-24T06:31:36.5910339Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.5910594Z                found type `panic::Location<'_>`
2019-10-24T06:31:36.5910633Z 
2019-10-24T06:31:36.5910633Z 
2019-10-24T06:31:36.7103695Z error[E0308]: mismatched types
2019-10-24T06:31:36.7104071Z   --> src/libcore/macros.rs:32:80
2019-10-24T06:31:36.7104329Z    |
2019-10-24T06:31:36.7104857Z 7  | / macro_rules! panic {
2019-10-24T06:31:36.7105214Z 8  | |     () => (
2019-10-24T06:31:36.7105560Z 9  | |         $crate::panic!("explicit panic")
2019-10-24T06:31:36.7106114Z ...  |
2019-10-24T06:31:36.7106114Z ...  |
2019-10-24T06:31:36.7106487Z 32 | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:36.7107206Z    | |                                                                                |
2019-10-24T06:31:36.7107638Z    | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:36.7108188Z    | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:36.7108572Z 33 | |         }
2019-10-24T06:31:36.7108572Z 33 | |         }
2019-10-24T06:31:36.7108864Z 34 | |     });
2019-10-24T06:31:36.7109170Z 35 | | }
2019-10-24T06:31:36.7109480Z    | |_- in this expansion of `panic!`
2019-10-24T06:31:36.7109689Z ...
2019-10-24T06:31:36.7110005Z 59 | / macro_rules! assert_eq {
2019-10-24T06:31:36.7110330Z 60 | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:36.7110641Z 61 | |         match (&$left, &$right) {
2019-10-24T06:31:36.7111256Z 62 | |             (left_val, right_val) => {
2019-10-24T06:31:36.7111568Z ...  |
2019-10-24T06:31:36.7111941Z 67 | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:36.7112223Z 68 |     left: `{:?}`,
2019-10-24T06:31:36.7112539Z 69 | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:36.7113113Z ...
2019-10-24T06:31:36.7113391Z 91 | |     });
2019-10-24T06:31:36.7113687Z 92 | | }
2019-10-24T06:31:36.7114130Z    | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:36.7114130Z    | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:36.7114398Z    | 
2019-10-24T06:31:36.7114677Z   ::: src/libcore/num/diy_float.rs:76:9
2019-10-24T06:31:36.7114890Z    |
2019-10-24T06:31:36.7115195Z 76 |           assert_eq!(self.f << edelta >> edelta, self.f);
2019-10-24T06:31:36.7115801Z    |
2019-10-24T06:31:36.7116088Z    = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:36.7116357Z               found type `panic::Location<'_>`
2019-10-24T06:31:36.7116396Z 
2019-10-24T06:31:36.7116396Z 
2019-10-24T06:31:37.2914334Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-10-24T06:31:37.3861860Z error[E0308]: mismatched types
2019-10-24T06:31:37.3862616Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:37.3862878Z     |
2019-10-24T06:31:37.3863213Z 7   | / macro_rules! panic {
2019-10-24T06:31:37.3863849Z 8   | |     () => (
2019-10-24T06:31:37.3864185Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:37.3864748Z ...   |
2019-10-24T06:31:37.3864748Z ...   |
2019-10-24T06:31:37.3865143Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:37.3865975Z     | |                                                                                |
2019-10-24T06:31:37.3866507Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:37.3866906Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:37.3867231Z 33  | |         }
2019-10-24T06:31:37.3867231Z 33  | |         }
2019-10-24T06:31:37.3867520Z 34  | |     });
2019-10-24T06:31:37.3867802Z 35  | | }
2019-10-24T06:31:37.3868134Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:37.3868359Z     | 
2019-10-24T06:31:37.3868620Z    ::: src/libcore/char/methods.rs:461:17
2019-10-24T06:31:37.3868857Z     |
2019-10-24T06:31:37.3869156Z 461 | /                 panic!(
2019-10-24T06:31:37.3869654Z 462 | |                     "encode_utf8: need {} bytes to encode U+{:X}, but the buffer has {}",
2019-10-24T06:31:37.3870045Z 463 | |                     from_u32_unchecked(code).len_utf8(),
2019-10-24T06:31:37.3870368Z 464 | |                     code,
2019-10-24T06:31:37.3870704Z 465 | |                     dst.len(),
2019-10-24T06:31:37.3871323Z     | |_________________- in this macro invocation
2019-10-24T06:31:37.3871872Z     |
2019-10-24T06:31:37.3872270Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:37.3872530Z                found type `panic::Location<'_>`
2019-10-24T06:31:37.3872530Z                found type `panic::Location<'_>`
2019-10-24T06:31:37.3872591Z 
2019-10-24T06:31:37.4017647Z error[E0308]: mismatched types
2019-10-24T06:31:37.4018446Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:37.4018707Z     |
2019-10-24T06:31:37.4019125Z 7   | / macro_rules! panic {
2019-10-24T06:31:37.4019409Z 8   | |     () => (
2019-10-24T06:31:37.4019744Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:37.4020417Z ...   |
2019-10-24T06:31:37.4020417Z ...   |
2019-10-24T06:31:37.4020768Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:37.4021868Z     | |                                                                                |
2019-10-24T06:31:37.4022301Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:37.4022712Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:37.4023007Z 33  | |         }
2019-10-24T06:31:37.4023007Z 33  | |         }
2019-10-24T06:31:37.4023297Z 34  | |     });
---
2019-10-24T06:31:37.8794173Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:37.8794406Z      |
2019-10-24T06:31:37.8794704Z 7    | / macro_rules! panic {
2019-10-24T06:31:37.8794996Z 8    | |     () => (
2019-10-24T06:31:37.8795341Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:37.8795941Z ...    |
2019-10-24T06:31:37.8795941Z ...    |
2019-10-24T06:31:37.8796311Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:37.8797040Z      | |                                                                                |
2019-10-24T06:31:37.8797592Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:37.8798051Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:37.8798364Z 33   | |         }
2019-10-24T06:31:37.8798364Z 33   | |         }
2019-10-24T06:31:37.8798651Z 34   | |     });
2019-10-24T06:31:37.8798947Z 35   | | }
2019-10-24T06:31:37.8799247Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:37.8799467Z      | 
2019-10-24T06:31:37.8799744Z     ::: src/libcore/option.rs:1193:5
2019-10-24T06:31:37.8799962Z      |
2019-10-24T06:31:37.8800540Z 1193 |       panic!("{}: {:?}", msg, value)
2019-10-24T06:31:37.8801405Z      |
2019-10-24T06:31:37.8801696Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:37.8802263Z                 found type `panic::Location<'_>`
2019-10-24T06:31:37.8802312Z 
2019-10-24T06:31:37.8802312Z 
2019-10-24T06:31:37.9461480Z error[E0308]: mismatched types
2019-10-24T06:31:37.9462256Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:37.9462600Z      |
2019-10-24T06:31:37.9462916Z 7    | / macro_rules! panic {
2019-10-24T06:31:37.9463240Z 8    | |     () => (
2019-10-24T06:31:37.9463568Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:37.9464144Z ...    |
2019-10-24T06:31:37.9464144Z ...    |
2019-10-24T06:31:37.9464503Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:37.9465241Z      | |                                                                                |
2019-10-24T06:31:37.9465641Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:37.9467262Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:37.9467600Z 33   | |         }
2019-10-24T06:31:37.9467600Z 33   | |         }
2019-10-24T06:31:37.9468154Z 34   | |     });
2019-10-24T06:31:37.9468500Z 35   | | }
2019-10-24T06:31:37.9468803Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:37.9469049Z      | 
2019-10-24T06:31:37.9469313Z     ::: src/libcore/result.rs:1165:5
2019-10-24T06:31:37.9469530Z      |
2019-10-24T06:31:37.9469829Z 1165 |       panic!("{}: {:?}", msg, error)
2019-10-24T06:31:37.9470386Z      |
2019-10-24T06:31:37.9470700Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:37.9470956Z                 found type `panic::Location<'_>`
2019-10-24T06:31:37.9470995Z 
2019-10-24T06:31:37.9470995Z 
2019-10-24T06:31:38.0265274Z error[E0308]: mismatched types
2019-10-24T06:31:38.0266084Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.0266912Z      |
2019-10-24T06:31:38.0267467Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.0267959Z 8    | |     () => (
2019-10-24T06:31:38.0268507Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.0269450Z ...    |
2019-10-24T06:31:38.0269450Z ...    |
2019-10-24T06:31:38.0270013Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.0271137Z      | |                                                                                |
2019-10-24T06:31:38.0271745Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.0272807Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.0273421Z 33   | |         }
2019-10-24T06:31:38.0273421Z 33   | |         }
2019-10-24T06:31:38.0273931Z 34   | |     });
2019-10-24T06:31:38.0274423Z 35   | | }
2019-10-24T06:31:38.0274919Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.0275316Z ...
2019-10-24T06:31:38.0275833Z 59   | / macro_rules! assert_eq {
2019-10-24T06:31:38.0276515Z 60   | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:38.0277140Z 61   | |         match (&$left, &$right) {
2019-10-24T06:31:38.0277651Z 62   | |             (left_val, right_val) => {
2019-10-24T06:31:38.0278117Z ...    |
2019-10-24T06:31:38.0278652Z 84   | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:38.0279110Z 85   |     left: `{:?}`,
2019-10-24T06:31:38.0279626Z 86   |    right: `{:?}`: {}"#, &*left_val, &*right_val,
2019-10-24T06:31:38.0280216Z 87   | |                            $crate::format_args!($($arg)+))
2019-10-24T06:31:38.0281470Z ...
2019-10-24T06:31:38.0282267Z 91   | |     });
2019-10-24T06:31:38.0282847Z 92   | | }
2019-10-24T06:31:38.0283349Z      | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:38.0283349Z      | |_- in this expansion of `assert_eq!`
2019-10-24T06:31:38.0283791Z      | 
2019-10-24T06:31:38.0284231Z     ::: src/libcore/slice/mod.rs:2137:9
2019-10-24T06:31:38.0284660Z      |
2019-10-24T06:31:38.0285215Z 2137 | /         assert_eq!(self.len(), src.len(),
2019-10-24T06:31:38.0285757Z 2138 | |                    "destination and source slices have different lengths");
2019-10-24T06:31:38.0288006Z      |
2019-10-24T06:31:38.0288698Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.0289185Z                 found type `panic::Location<'_>`
2019-10-24T06:31:38.0289364Z 
2019-10-24T06:31:38.0289364Z 
2019-10-24T06:31:38.0740010Z error[E0308]: mismatched types
2019-10-24T06:31:38.0741202Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.0741752Z      |
2019-10-24T06:31:38.0742601Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.0743207Z 8    | |     () => (
2019-10-24T06:31:38.0743779Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.0744723Z ...    |
2019-10-24T06:31:38.0744723Z ...    |
2019-10-24T06:31:38.0745494Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.0746718Z      | |                                                                                |
2019-10-24T06:31:38.0747331Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.0747944Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.0748471Z 33   | |         }
2019-10-24T06:31:38.0748471Z 33   | |         }
2019-10-24T06:31:38.0748979Z 34   | |     });
2019-10-24T06:31:38.0749453Z 35   | | }
2019-10-24T06:31:38.0750191Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.0750615Z      | 
2019-10-24T06:31:38.0751097Z     ::: src/libcore/slice/mod.rs:2584:5
2019-10-24T06:31:38.0751502Z      |
2019-10-24T06:31:38.0752010Z 2584 |       panic!("index {} out of range for slice of length {}", index, len);
2019-10-24T06:31:38.0753528Z      |
2019-10-24T06:31:38.0754054Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.0754511Z                 found type `panic::Location<'_>`
2019-10-24T06:31:38.0754682Z 
2019-10-24T06:31:38.0754682Z 
2019-10-24T06:31:38.0755200Z error[E0308]: mismatched types
2019-10-24T06:31:38.0755457Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.0755941Z      |
2019-10-24T06:31:38.0756446Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.0756939Z 8    | |     () => (
2019-10-24T06:31:38.0757479Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.0758412Z ...    |
2019-10-24T06:31:38.0758412Z ...    |
2019-10-24T06:31:38.0758959Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.0760207Z      | |                                                                                |
2019-10-24T06:31:38.0760865Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.0761500Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.0776774Z 33   | |         }
2019-10-24T06:31:38.0776774Z 33   | |         }
2019-10-24T06:31:38.0777405Z 34   | |     });
2019-10-24T06:31:38.0777911Z 35   | | }
2019-10-24T06:31:38.0778478Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.0778908Z      | 
2019-10-24T06:31:38.0779381Z     ::: src/libcore/slice/mod.rs:2590:5
2019-10-24T06:31:38.0779799Z      |
2019-10-24T06:31:38.0780305Z 2590 |       panic!("slice index starts at {} but ends at {}", index, end);
2019-10-24T06:31:38.0781617Z      |
2019-10-24T06:31:38.0782620Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.0783160Z                 found type `panic::Location<'_>`
2019-10-24T06:31:38.0783337Z 
2019-10-24T06:31:38.0783337Z 
2019-10-24T06:31:38.3309334Z error[E0308]: mismatched types
2019-10-24T06:31:38.3309838Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.3310072Z     |
2019-10-24T06:31:38.3310396Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.3310719Z 8   | |     () => (
2019-10-24T06:31:38.3311064Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.3311597Z ...   |
2019-10-24T06:31:38.3311597Z ...   |
2019-10-24T06:31:38.3311985Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.3313211Z     | |                                                                                |
2019-10-24T06:31:38.3313629Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.3314379Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.3314928Z 33  | |         }
2019-10-24T06:31:38.3314928Z 33  | |         }
2019-10-24T06:31:38.3315269Z 34  | |     });
2019-10-24T06:31:38.3315569Z 35  | | }
2019-10-24T06:31:38.3315889Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:38.3316409Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.3316409Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.3316721Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:38.3317037Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:38.3317368Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:38.3317609Z ...   |
2019-10-24T06:31:38.3317944Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:38.3318396Z 68  |     left: `{:?}`,
2019-10-24T06:31:38.3318714Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:38.3319277Z ...
2019-10-24T06:31:38.3319563Z 91  | |     });
2019-10-24T06:31:38.3319856Z 92  | | }
2019-10-24T06:31:38.3320174Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.3320174Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.3320388Z ...
2019-10-24T06:31:38.3320701Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:38.3321064Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:38.3321806Z 226 | | }
2019-10-24T06:31:38.3322469Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:38.3322755Z     | 
2019-10-24T06:31:38.3323010Z    ::: src/libcore/slice/sort.rs:345:9
2019-10-24T06:31:38.3323010Z    ::: src/libcore/slice/sort.rs:345:9
2019-10-24T06:31:38.3323223Z     |
2019-10-24T06:31:38.3323534Z 345 |           debug_assert_eq!(width(l, r), block_l);
2019-10-24T06:31:38.3324261Z     |
2019-10-24T06:31:38.3324604Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.3324860Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.3324920Z 
2019-10-24T06:31:38.3324920Z 
2019-10-24T06:31:38.3566688Z error[E0308]: mismatched types
2019-10-24T06:31:38.3567224Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.3567498Z     |
2019-10-24T06:31:38.3567805Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.3568102Z 8   | |     () => (
2019-10-24T06:31:38.3568643Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.3569275Z ...   |
2019-10-24T06:31:38.3569275Z ...   |
2019-10-24T06:31:38.3569637Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.3570705Z     | |                                                                                |
2019-10-24T06:31:38.3571116Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.3571524Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.3571826Z 33  | |         }
2019-10-24T06:31:38.3571826Z 33  | |         }
2019-10-24T06:31:38.3572480Z 34  | |     });
2019-10-24T06:31:38.3572827Z 35  | | }
2019-10-24T06:31:38.3573159Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:38.3574757Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.3574757Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.3575129Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:38.3575444Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:38.3575769Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:38.3576031Z ...   |
2019-10-24T06:31:38.3576368Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:38.3576640Z 68  |     left: `{:?}`,
2019-10-24T06:31:38.3577172Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:38.3577801Z ...
2019-10-24T06:31:38.3578083Z 91  | |     });
2019-10-24T06:31:38.3578370Z 92  | | }
2019-10-24T06:31:38.3578700Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.3578700Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.3578911Z ...
2019-10-24T06:31:38.3579209Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:38.3579594Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:38.3580455Z 226 | | }
2019-10-24T06:31:38.3580778Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:38.3581045Z     | 
2019-10-24T06:31:38.3581351Z    ::: src/libcore/slice/sort.rs:357:9
2019-10-24T06:31:38.3581351Z    ::: src/libcore/slice/sort.rs:357:9
2019-10-24T06:31:38.3581592Z     |
2019-10-24T06:31:38.3581930Z 357 |           debug_assert_eq!(width(l, r), block_r);
2019-10-24T06:31:38.3583074Z     |
2019-10-24T06:31:38.3583420Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.3583686Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.3583728Z 
2019-10-24T06:31:38.3583728Z 
2019-10-24T06:31:38.4061192Z error[E0308]: mismatched types
2019-10-24T06:31:38.4061681Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.4061917Z     |
2019-10-24T06:31:38.4062586Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.4062962Z 8   | |     () => (
2019-10-24T06:31:38.4063281Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.4063868Z ...   |
2019-10-24T06:31:38.4063868Z ...   |
2019-10-24T06:31:38.4064241Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.4065211Z     | |                                                                                |
2019-10-24T06:31:38.4065687Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.4066108Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.4066413Z 33  | |         }
2019-10-24T06:31:38.4066413Z 33  | |         }
2019-10-24T06:31:38.4066699Z 34  | |     });
2019-10-24T06:31:38.4067002Z 35  | | }
2019-10-24T06:31:38.4067306Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:38.4067528Z     | 
2019-10-24T06:31:38.4067804Z    ::: src/libcore/slice/sort.rs:757:9
2019-10-24T06:31:38.4068182Z     |
2019-10-24T06:31:38.4068521Z 757 |           panic!("partition_at_index index {} greater than length of slice {}", index, v.len());
2019-10-24T06:31:38.4089354Z     |
2019-10-24T06:31:38.4089950Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.4090241Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.4090300Z 
2019-10-24T06:31:38.4090300Z 
2019-10-24T06:31:38.5214795Z error[E0308]: mismatched types
2019-10-24T06:31:38.5216331Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.5218263Z      |
2019-10-24T06:31:38.5219550Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.5220778Z 8    | |     () => (
2019-10-24T06:31:38.5238780Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.5239417Z ...    |
2019-10-24T06:31:38.5239417Z ...    |
2019-10-24T06:31:38.5239784Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.5240531Z      | |                                                                                |
2019-10-24T06:31:38.5240950Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.5241547Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.5241927Z 33   | |         }
2019-10-24T06:31:38.5241927Z 33   | |         }
2019-10-24T06:31:38.5242224Z 34   | |     });
2019-10-24T06:31:38.5242927Z 35   | | }
2019-10-24T06:31:38.5243265Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.5243488Z      | 
2019-10-24T06:31:38.5243739Z     ::: src/libcore/str/mod.rs:2051:9
2019-10-24T06:31:38.5243975Z      |
2019-10-24T06:31:38.5244326Z 2051 |           panic!("byte index {} is out of bounds of `{}`{}", oob_index, s_trunc, ellipsis);
2019-10-24T06:31:38.5245201Z      |
2019-10-24T06:31:38.5245496Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.5245779Z                 found type `panic::Location<'_>`
2019-10-24T06:31:38.5245820Z 
2019-10-24T06:31:38.5245820Z 
2019-10-24T06:31:38.5390957Z error[E0308]: mismatched types
2019-10-24T06:31:38.5391345Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.5391573Z      |
2019-10-24T06:31:38.5391901Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.5393109Z 8    | |     () => (
2019-10-24T06:31:38.5393452Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.5394039Z ...    |
2019-10-24T06:31:38.5394039Z ...    |
2019-10-24T06:31:38.5394401Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.5395145Z      | |                                                                                |
2019-10-24T06:31:38.5395560Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.5396221Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.5396562Z 33   | |         }
2019-10-24T06:31:38.5396562Z 33   | |         }
2019-10-24T06:31:38.5396851Z 34   | |     });
2019-10-24T06:31:38.5397365Z 35   | | }
2019-10-24T06:31:38.5397748Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.5397957Z ...
2019-10-24T06:31:38.5398255Z 1226 | /     macro_rules! assert {
2019-10-24T06:31:38.5398619Z 1227 | |         ($cond:expr) => ({ /* compiler built-in */ });
2019-10-24T06:31:38.5398951Z 1228 | |         ($cond:expr,) => ({ /* compiler built-in */ });
2019-10-24T06:31:38.5399310Z 1229 | |         ($cond:expr, $($arg:tt)+) => ({ /* compiler built-in */ })
2019-10-24T06:31:38.5399918Z      | |_____- in this expansion of `assert!`
2019-10-24T06:31:38.5400158Z      | 
2019-10-24T06:31:38.5400565Z     ::: src/libcore/str/mod.rs:2055:5
2019-10-24T06:31:38.5400781Z      |
2019-10-24T06:31:38.5400781Z      |
2019-10-24T06:31:38.5401125Z 2055 |       assert!(begin <= end, "begin <= end ({} <= {}) when slicing `{}`{}",
2019-10-24T06:31:38.5401624Z      | |_____|
2019-10-24T06:31:38.5401882Z      | |
2019-10-24T06:31:38.5401882Z      | |
2019-10-24T06:31:38.5402541Z 2056 | |             begin, end, s_trunc, ellipsis);
2019-10-24T06:31:38.5403222Z      | |___________________________________________|
2019-10-24T06:31:38.5403558Z      | |___________________________________________in this macro invocation
2019-10-24T06:31:38.5403888Z      |                                             in this macro invocation
2019-10-24T06:31:38.5404111Z      |
---
2019-10-24T06:31:38.5414034Z     --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.5414272Z      |
2019-10-24T06:31:38.5414594Z 7    | / macro_rules! panic {
2019-10-24T06:31:38.5414906Z 8    | |     () => (
2019-10-24T06:31:38.5415227Z 9    | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.5415981Z ...    |
2019-10-24T06:31:38.5415981Z ...    |
2019-10-24T06:31:38.5416402Z 32   | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.5417152Z      | |                                                                                |
2019-10-24T06:31:38.5417548Z      | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.5417975Z      | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.5418282Z 33   | |         }
2019-10-24T06:31:38.5418282Z 33   | |         }
2019-10-24T06:31:38.5418594Z 34   | |     });
2019-10-24T06:31:38.5419023Z 35   | | }
2019-10-24T06:31:38.5419326Z      | |_- in this expansion of `panic!`
2019-10-24T06:31:38.5419572Z      | 
2019-10-24T06:31:38.5419824Z     ::: src/libcore/str/mod.rs:2068:5
2019-10-24T06:31:38.5420040Z      |
2019-10-24T06:31:38.5420432Z 2068 | /     panic!("byte index {} is not a char boundary; it is inside {:?} (bytes {:?}) of `{}`{}",
2019-10-24T06:31:38.5420771Z 2069 | |            index, ch, char_range, s_trunc, ellipsis);
2019-10-24T06:31:38.5421375Z      |
2019-10-24T06:31:38.5421663Z      = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.5421932Z                 found type `panic::Location<'_>`
2019-10-24T06:31:38.5421970Z 
2019-10-24T06:31:38.5421970Z 
2019-10-24T06:31:38.7429874Z error[E0308]: mismatched types
2019-10-24T06:31:38.7430275Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.7430508Z     |
2019-10-24T06:31:38.7430830Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.7431129Z 8   | |     () => (
2019-10-24T06:31:38.7431489Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.7432024Z ...   |
2019-10-24T06:31:38.7432024Z ...   |
2019-10-24T06:31:38.7432692Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.7434291Z     | |                                                                                |
2019-10-24T06:31:38.7434736Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.7435125Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.7435441Z 33  | |         }
2019-10-24T06:31:38.7435441Z 33  | |         }
2019-10-24T06:31:38.7435736Z 34  | |     });
2019-10-24T06:31:38.7436020Z 35  | | }
2019-10-24T06:31:38.7436362Z     | |_- in this expansion of `panic!` (#3)
2019-10-24T06:31:38.7437019Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.7437019Z 59  | / macro_rules! assert_eq {
2019-10-24T06:31:38.7437351Z 60  | |     ($left:expr, $right:expr) => ({
2019-10-24T06:31:38.7437660Z 61  | |         match (&$left, &$right) {
2019-10-24T06:31:38.7438003Z 62  | |             (left_val, right_val) => {
2019-10-24T06:31:38.7438251Z ...   |
2019-10-24T06:31:38.7438587Z 67  | /                     panic!(r#"assertion failed: `(left == right)`
2019-10-24T06:31:38.7438937Z 68  |     left: `{:?}`,
2019-10-24T06:31:38.7439285Z 69  | |  right: `{:?}`"#, &*left_val, &*right_val)
2019-10-24T06:31:38.7439856Z ...
2019-10-24T06:31:38.7440133Z 91  | |     });
2019-10-24T06:31:38.7440421Z 92  | | }
2019-10-24T06:31:38.7440758Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.7440758Z     | |_- in this expansion of `$crate::assert_eq!` (#2)
2019-10-24T06:31:38.7440964Z ...
2019-10-24T06:31:38.7441261Z 224 | / macro_rules! debug_assert_eq {
2019-10-24T06:31:38.7441655Z 225 | |     ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert_eq!($($arg)*); })
2019-10-24T06:31:38.7442770Z 226 | | }
2019-10-24T06:31:38.7443241Z     | |_- in this expansion of `debug_assert_eq!` (#1)
2019-10-24T06:31:38.7443504Z     | 
2019-10-24T06:31:38.7443763Z    ::: src/libcore/hash/sip.rs:130:5
2019-10-24T06:31:38.7443763Z    ::: src/libcore/hash/sip.rs:130:5
2019-10-24T06:31:38.7443989Z     |
2019-10-24T06:31:38.7444285Z 130 |       debug_assert_eq!(i, len);
2019-10-24T06:31:38.7444838Z     |
2019-10-24T06:31:38.7445144Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.7445404Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.7445444Z 
2019-10-24T06:31:38.7445444Z 
2019-10-24T06:31:38.9019779Z error[E0308]: mismatched types
2019-10-24T06:31:38.9020732Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.9021235Z     |
2019-10-24T06:31:38.9022104Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.9023043Z 8   | |     () => (
2019-10-24T06:31:38.9023705Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.9024685Z ...   |
2019-10-24T06:31:38.9024685Z ...   |
2019-10-24T06:31:38.9025200Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.9026414Z     | |                                                                                |
2019-10-24T06:31:38.9027083Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.9027694Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.9028200Z 33  | |         }
2019-10-24T06:31:38.9028200Z 33  | |         }
2019-10-24T06:31:38.9028709Z 34  | |     });
2019-10-24T06:31:38.9029206Z 35  | | }
2019-10-24T06:31:38.9029735Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:38.9030159Z     | 
2019-10-24T06:31:38.9030630Z    ::: src/libcore/fmt/num.rs:108:1
2019-10-24T06:31:38.9031534Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9031534Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9032287Z 109 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
2019-10-24T06:31:38.9033207Z 110 | |         impl GenericRadix for $T {
2019-10-24T06:31:38.9033762Z 111 | |             const BASE: u8 = $base;
2019-10-24T06:31:38.9034230Z ...   |
2019-10-24T06:31:38.9034793Z 116 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
2019-10-24T06:31:38.9035908Z ...   |
2019-10-24T06:31:38.9036392Z 120 | |     }
2019-10-24T06:31:38.9036874Z 121 | | }
2019-10-24T06:31:38.9036874Z 121 | | }
2019-10-24T06:31:38.9037585Z     | |_- in this expansion of `radix!`
2019-10-24T06:31:38.9038045Z 122 | 
2019-10-24T06:31:38.9038583Z 123 |   radix! { Binary,    2, "0b", x @  0 ..=  1 => b'0' + x }
2019-10-24T06:31:38.9039616Z     |
2019-10-24T06:31:38.9039974Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.9040435Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.9040619Z 
2019-10-24T06:31:38.9040619Z 
2019-10-24T06:31:38.9181994Z error[E0308]: mismatched types
2019-10-24T06:31:38.9183794Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.9184296Z     |
2019-10-24T06:31:38.9184810Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.9185337Z 8   | |     () => (
2019-10-24T06:31:38.9185883Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.9186865Z ...   |
2019-10-24T06:31:38.9186865Z ...   |
2019-10-24T06:31:38.9187426Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.9188564Z     | |                                                                                |
2019-10-24T06:31:38.9189355Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.9190063Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.9190578Z 33  | |         }
2019-10-24T06:31:38.9190578Z 33  | |         }
2019-10-24T06:31:38.9191092Z 34  | |     });
2019-10-24T06:31:38.9191566Z 35  | | }
2019-10-24T06:31:38.9192081Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:38.9192870Z     | 
2019-10-24T06:31:38.9193403Z    ::: src/libcore/fmt/num.rs:108:1
2019-10-24T06:31:38.9194348Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9194348Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9194910Z 109 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
2019-10-24T06:31:38.9195677Z 110 | |         impl GenericRadix for $T {
2019-10-24T06:31:38.9196210Z 111 | |             const BASE: u8 = $base;
2019-10-24T06:31:38.9196655Z ...   |
2019-10-24T06:31:38.9197221Z 116 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
2019-10-24T06:31:38.9198302Z ...   |
2019-10-24T06:31:38.9198818Z 120 | |     }
2019-10-24T06:31:38.9199297Z 121 | | }
2019-10-24T06:31:38.9199297Z 121 | | }
2019-10-24T06:31:38.9199821Z     | |_- in this expansion of `radix!`
2019-10-24T06:31:38.9200222Z ...
2019-10-24T06:31:38.9200749Z 124 |   radix! { Octal,     8, "0o", x @  0 ..=  7 => b'0' + x }
2019-10-24T06:31:38.9201752Z     |
2019-10-24T06:31:38.9202112Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.9202906Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.9203139Z 
2019-10-24T06:31:38.9203139Z 
2019-10-24T06:31:38.9203555Z error[E0308]: mismatched types
2019-10-24T06:31:38.9203810Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.9204045Z     |
2019-10-24T06:31:38.9204578Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.9205238Z 8   | |     () => (
2019-10-24T06:31:38.9205842Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.9206876Z ...   |
2019-10-24T06:31:38.9206876Z ...   |
2019-10-24T06:31:38.9207416Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.9208723Z     | |                                                                                |
2019-10-24T06:31:38.9209391Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.9210002Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.9210719Z 33  | |         }
2019-10-24T06:31:38.9210719Z 33  | |         }
2019-10-24T06:31:38.9211227Z 34  | |     });
2019-10-24T06:31:38.9211704Z 35  | | }
2019-10-24T06:31:38.9212202Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:38.9213053Z     | 
2019-10-24T06:31:38.9213581Z    ::: src/libcore/fmt/num.rs:108:1
2019-10-24T06:31:38.9214508Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9214508Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9215070Z 109 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
2019-10-24T06:31:38.9215619Z 110 | |         impl GenericRadix for $T {
2019-10-24T06:31:38.9216126Z 111 | |             const BASE: u8 = $base;
2019-10-24T06:31:38.9216603Z ...   |
2019-10-24T06:31:38.9217155Z 116 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
2019-10-24T06:31:38.9218236Z ...   |
2019-10-24T06:31:38.9218740Z 120 | |     }
2019-10-24T06:31:38.9219219Z 121 | | }
2019-10-24T06:31:38.9219219Z 121 | | }
2019-10-24T06:31:38.9219835Z     | |_- in this expansion of `radix!`
2019-10-24T06:31:38.9220317Z ...
2019-10-24T06:31:38.9220836Z 125 | / radix! { LowerHex, 16, "0x", x @  0 ..=  9 => b'0' + x,
2019-10-24T06:31:38.9221349Z 126 | |                              x @ 10 ..= 15 => b'a' + (x - 10) }
2019-10-24T06:31:38.9222303Z     |
2019-10-24T06:31:38.9223085Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.9223523Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.9251729Z 
2019-10-24T06:31:38.9251729Z 
2019-10-24T06:31:38.9378079Z error[E0308]: mismatched types
2019-10-24T06:31:38.9378841Z    --> src/libcore/macros.rs:32:80
2019-10-24T06:31:38.9379309Z     |
2019-10-24T06:31:38.9379816Z 7   | / macro_rules! panic {
2019-10-24T06:31:38.9380628Z 8   | |     () => (
2019-10-24T06:31:38.9381022Z 9   | |         $crate::panic!("explicit panic")
2019-10-24T06:31:38.9381991Z ...   |
2019-10-24T06:31:38.9381991Z ...   |
2019-10-24T06:31:38.9383004Z 32  | |             $crate::panicking::panic_fmt($crate::format_args!($fmt, $($arg)+), loc)
2019-10-24T06:31:38.9384315Z     | |                                                                                |
2019-10-24T06:31:38.9384941Z     | |                                                                                expected reference, found struct `panic::Location`
2019-10-24T06:31:38.9385552Z     | |                                                                                help: consider borrowing here: `&loc`
2019-10-24T06:31:38.9386074Z 33  | |         }
2019-10-24T06:31:38.9386074Z 33  | |         }
2019-10-24T06:31:38.9386588Z 34  | |     });
2019-10-24T06:31:38.9387074Z 35  | | }
2019-10-24T06:31:38.9387581Z     | |_- in this expansion of `panic!`
2019-10-24T06:31:38.9388023Z     | 
2019-10-24T06:31:38.9388466Z    ::: src/libcore/fmt/num.rs:108:1
2019-10-24T06:31:38.9389481Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9389481Z 108 | / macro_rules! radix {
2019-10-24T06:31:38.9390169Z 109 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
2019-10-24T06:31:38.9390777Z 110 | |         impl GenericRadix for $T {
2019-10-24T06:31:38.9391279Z 111 | |             const BASE: u8 = $base;
2019-10-24T06:31:38.9391758Z ...   |
2019-10-24T06:31:38.9392296Z 116 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
2019-10-24T06:31:38.9394212Z ...   |
2019-10-24T06:31:38.9394719Z 120 | |     }
2019-10-24T06:31:38.9395212Z 121 | | }
2019-10-24T06:31:38.9395212Z 121 | | }
2019-10-24T06:31:38.9395722Z     | |_- in this expansion of `radix!`
2019-10-24T06:31:38.9396425Z ...
2019-10-24T06:31:38.9396954Z 127 | / radix! { UpperHex, 16, "0x", x @  0 ..=  9 => b'0' + x,
2019-10-24T06:31:38.9397532Z 128 | |                              x @ 10 ..= 15 => b'A' + (x - 10) }
2019-10-24T06:31:38.9398541Z     |
2019-10-24T06:31:38.9399019Z     = note: expected type `&panic::Location<'_>`
2019-10-24T06:31:38.9399471Z                found type `panic::Location<'_>`
2019-10-24T06:31:38.9399658Z 
---
2019-10-24T06:31:42.2192632Z   local time: Thu Oct 24 06:31:42 UTC 2019
2019-10-24T06:31:42.3074114Z   network time: Thu, 24 Oct 2019 06:31:42 GMT
2019-10-24T06:31:42.3074977Z == end clock drift check ==
2019-10-24T06:31:43.2023173Z 
2019-10-24T06:31:43.2146012Z ##[error]Bash exited with code '1'.
2019-10-24T06:31:43.2181561Z ##[section]Starting: Checkout
2019-10-24T06:31:43.2183472Z ==============================================================================
2019-10-24T06:31:43.2183527Z Task         : Get sources
2019-10-24T06:31:43.2183592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
