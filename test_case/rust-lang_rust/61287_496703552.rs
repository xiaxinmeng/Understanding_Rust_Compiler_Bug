
[01:20:23] [0m[0m[1m[32m   Compiling[0m clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:20:28] [0m[1m[38;5;9merror[0m[0m[1m: no variant `Bits` in enum `rustc_mir::interpret::Scalar<_, _>`[0m
[01:20:28] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/clippy/clippy_lints/src/consts.rs:475:36[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m|[0m
[01:20:28] [0m[1m[38;5;12m475[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        ConstValue::Scalar(Scalar::Bits { bits: b, .. }) => match result.ty.sty {[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12m--------[0m[0m[1m[38;5;9m^^^^[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12m|[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12mvariant not found in `rustc_mir::interpret::Scalar<_, _>`[0m
[01:20:28] 
[01:20:28] [0m[1m[38;5;9merror[0m[0m[1m: no variant `Bits` in enum `rustc_mir::interpret::Scalar<_, _>`[0m
[01:20:28] [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/tools/clippy/clippy_lints/src/consts.rs:475:36[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m|[0m
[01:20:28] [0m[1m[38;5;12m475[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        ConstValue::Scalar(Scalar::Bits { bits: b, .. }) => match result.ty.sty {[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12m--------[0m[0m[1m[38;5;9m^^^^[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12m|[0m
[01:20:28] [0m    [0m[0m[1m[38;5;12m| [0m[0m                           [0m[0m[1m[38;5;12mvariant not found in `rustc_mir::interpret::Scalar<_, _>`[0m
[01:20:28] 
[01:20:31] [0m[1m[38;5;9merror[0m[0m[1m: aborting due to previous error[0m
