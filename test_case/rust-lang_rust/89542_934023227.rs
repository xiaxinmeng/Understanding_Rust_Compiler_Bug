plain
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: `Duration::as_secs_f32` is not yet stable as a const fn
    |
    |
377 |     const SECONDS_F32: f32 = Duration::SECOND.as_secs_f32();
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::from_secs_f32` is not yet stable as a const fn
    |
    |
380 |     const FROM_SECONDS_F32: Duration = Duration::from_secs_f32(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::as_secs_f64` is not yet stable as a const fn
    |
    |
383 |     const SECONDS_F64: f64 = Duration::SECOND.as_secs_f64();
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::from_secs_f64` is not yet stable as a const fn
    |
    |
386 |     const FROM_SECONDS_F64: Duration = Duration::from_secs_f64(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::mul_f32` is not yet stable as a const fn
    |
    |
418 |     const MUL_F32: Duration = Duration::SECOND.mul_f32(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::mul_f64` is not yet stable as a const fn
    |
    |
421 |     const MUL_F64: Duration = Duration::SECOND.mul_f64(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::div_f32` is not yet stable as a const fn
    |
    |
427 |     const DIV_F32: Duration = Duration::SECOND.div_f32(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::div_f64` is not yet stable as a const fn
    |
    |
430 |     const DIV_F64: Duration = Duration::SECOND.div_f64(1.0);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::div_duration_f32` is not yet stable as a const fn
    |
    |
433 |     const DIV_DURATION_F32: f32 = Duration::SECOND.div_duration_f32(Duration::SECOND);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable

error: `Duration::div_duration_f64` is not yet stable as a const fn
    |
    |
436 |     const DIV_DURATION_F64: f64 = Duration::SECOND.div_duration_f64(Duration::SECOND);
    |
    |
    = help: add `#![feature(duration_consts_float)]` to the crate attributes to enable
error: could not compile `core` due to 10 previous errors
Build completed unsuccessfully in 0:01:40
