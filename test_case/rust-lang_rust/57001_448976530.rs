
warning[E0170]: pattern binding `Sleep` is named the same as one of the variants of the type `SleepMode`
 --> src/lib.rs:9:9
  |
9 |         Sleep => {
  |         ^^^^^ help: to match on the variant, qualify the path: `SleepMode::Sleep`

warning[E0170]: pattern binding `Stop` is named the same as one of the variants of the type `SleepMode`
  --> src/lib.rs:12:9
   |
12 |         Stop => {
   |         ^^^^ help: to match on the variant, qualify the path: `SleepMode::Stop`

warning[E0170]: pattern binding `Standby` is named the same as one of the variants of the type `SleepMode`
  --> src/lib.rs:15:9
   |
15 |         Standby => {
   |         ^^^^^^^ help: to match on the variant, qualify the path: `SleepMode::Standby`

warning: unreachable pattern
  --> src/lib.rs:12:9
   |
9  |         Sleep => {
   |         ----- matches any value
...
12 |         Stop => {
   |         ^^^^ unreachable pattern
   |
   = note: #[warn(unreachable_patterns)] on by default

warning: unreachable pattern
  --> src/lib.rs:15:9
   |
9  |         Sleep => {
   |         ----- matches any value
...
15 |         Standby => {
   |         ^^^^^^^ unreachable pattern

warning: unused variable: `Sleep`
 --> src/lib.rs:9:9
  |
9 |         Sleep => {
  |         ^^^^^ help: consider using `_Sleep` instead
  |
  = note: #[warn(unused_variables)] on by default

warning: unused variable: `Stop`
  --> src/lib.rs:12:9
   |
12 |         Stop => {
   |         ^^^^ help: consider using `_Stop` instead

warning: unused variable: `Standby`
  --> src/lib.rs:15:9
   |
15 |         Standby => {
   |         ^^^^^^^ help: consider using `_Standby` instead

warning: enum is never used: `SleepMode`
 --> src/lib.rs:1:1
  |
1 | enum SleepMode {
  | ^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `sleep`
 --> src/lib.rs:7:1
  |
7 | fn sleep(sleep_mode: SleepMode) {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable `Sleep` should have a snake case name such as `sleep`
 --> src/lib.rs:9:9
  |
9 |         Sleep => {
  |         ^^^^^
  |
  = note: #[warn(non_snake_case)] on by default

warning: variable `Stop` should have a snake case name such as `stop`
  --> src/lib.rs:12:9
   |
12 |         Stop => {
   |         ^^^^

warning: variable `Standby` should have a snake case name such as `standby`
  --> src/lib.rs:15:9
   |
15 |         Standby => {
   |         ^^^^^^^
