plain
[00:04:02]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:04:09] warning: constant item is never used: `NS`
[00:04:09]   --> libcore/time.rs:36:1
[00:04:09]    |
[00:04:09] 36 | const NS: Duration = Duration::from_nanos(1);
[00:04:09]    |
[00:04:09]    = note: #[warn(dead_code)] on by default
[00:04:09] 
[00:04:09] warning: constant item is never used: `US`
[00:04:09] warning: constant item is never used: `US`
[00:04:09]   --> libcore/time.rs:39:1
[00:04:09]    |
[00:04:09] 39 | const US: Duration = Duration::from_micros(1);
[00:04:09] 
[00:04:09] warning: constant item is never used: `MS`
[00:04:09]   --> libcore/time.rs:42:1
[00:04:09]    |
[00:04:09]    |
[00:04:09] 42 | const MS: Duration = Duration::from_millis(1);
[00:04:09] 
[00:04:09] warning: constant item is never used: `S`
[00:04:09]   --> libcore/time.rs:45:1
[00:04:09]    |
[00:04:09]    |
[00:04:09] 45 | const S: Duration = Duration::from_secs(1);
[00:04:09] 
[00:04:09] error: This node does not have a stability attribute
[00:04:09]    --> libcore/time.rs:517:1
[00:04:09]     |
[00:04:09]     |
[00:04:09] 517 | / impl Mul<Duration> for u32 {
[00:04:09] 518 | |     type Output = Duration;
[00:04:09] 519 | |
[00:04:09] 520 | |     fn mul(self, rhs: Duration) -> Duration {
[00:04:09] 521 | |         rhs.checked_mul(self).expect("overflow when multiplying scalar by duration")
[00:04:09] 523 | | }
[00:04:09]     | |_^
[00:04:09] 
[00:04:10] error: aborting due to previous error
