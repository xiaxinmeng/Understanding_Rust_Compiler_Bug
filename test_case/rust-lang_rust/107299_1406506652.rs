plain
24  | / rustc_index::newtype_index! {
25  | |     struct PreorderIndex {}
26  | | }
    | | ^
    | | |
    | |_expected `u32`, found pattern `u32 is ..=4294967040`
    |
   ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/library/core/src/cmp.rs:236:1
    |
236 |   pub macro PartialEq($item:item) {
236 |   pub macro PartialEq($item:item) {
    |   ------------------- in this expansion of `#[derive(PartialEq)]`
    = note:      expected type `u32`
    = note:      expected type `u32`
            found pattern type `u32 is ..=4294967040`
error[E0308]: mismatched types
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
    |
24  | / rustc_index::newtype_index! {
24  | / rustc_index::newtype_index! {
25  | |     struct PreorderIndex {}
26  | | }
    | | ^
    | | |
    | |_expected pattern `u32 is ..=4294967040`, found `u32`
    |
   ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/library/core/src/cmp.rs:236:1
    |
236 |   pub macro PartialEq($item:item) {
236 |   pub macro PartialEq($item:item) {
    |   ------------------- in this expansion of `#[derive(PartialEq)]`
    |
    = note: expected pattern type `u32 is ..=4294967040`

error[E0308]: mismatched types
    --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
     |
     |
24   | / rustc_index::newtype_index! {
25   | |     struct PreorderIndex {}
26   | | }
     | | ^
     | | |
     | | expected `u32`, found pattern `u32 is ..=4294967040`
     | |_arguments to this function are incorrect
     |
    ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/library/core/src/cmp.rs:1161:1
     |
1161 |   pub macro PartialOrd($item:item) {
1161 |   pub macro PartialOrd($item:item) {
     |   -------------------- in this expansion of `#[derive(PartialOrd)]`
     = note: expected reference `&u32`
     = note: expected reference `&u32`
                found reference `&u32 is ..=4294967040`
    --> /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/library/core/src/cmp.rs:1080:8
     |
     |
1080 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
   |
   |
24 | / rustc_index::newtype_index! {
25 | |     struct PreorderIndex {}
26 | | }
   | | ^
   | | |
   | |_expected pattern `u32 is ..=4294967040`, found `u32`
   |
  ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
   |
   |
53 |   pub fn newtype_index(input: TokenStream) -> TokenStream {
   |
   |
   = note: expected pattern type `u32 is ..=4294967040`

error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
   |
   |
24 | / rustc_index::newtype_index! {
25 | |     struct PreorderIndex {}
26 | | }
   | | ^
   | | |
   | | expected `u32`, found pattern `u32 is ..=4294967040`
   | |_expected `u32` because of return type
   |
  ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
   |
   |
53 |   pub fn newtype_index(input: TokenStream) -> TokenStream {
   |
   = note:      expected type `u32`
   = note:      expected type `u32`
           found pattern type `u32 is ..=4294967040`
error[E0308]: mismatched types
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
    |
24  | / rustc_index::newtype_index! {
24  | / rustc_index::newtype_index! {
25  | |     struct PreorderIndex {}
26  | | }
    | | ^
    | | |
    | | expected pattern `u32 is ..=4294967040`, found `u32`
    | |_arguments to this method are incorrect
    |
   ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
    |
    |
53  |   pub fn newtype_index(input: TokenStream) -> TokenStream {
    |
    |
    = note: expected pattern type `u32 is ..=4294967040`
help: the return type of this call is `u32` due to the type of the argument passed
   --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
    |
24  | / rustc_index::newtype_index! {
24  | / rustc_index::newtype_index! {
25  | |     struct PreorderIndex {}
26  | | }
    | | ^
    | | |
    | |_this argument influences the return type of `unwrap_or`
    |
   ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
    |
    |
53  |   pub fn newtype_index(input: TokenStream) -> TokenStream {
note: associated function defined here
   --> /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/library/core/src/option.rs:844:18
    |
844 |     pub const fn unwrap_or(self, default: T) -> T
---
24 | / rustc_index::newtype_index! {
25 | |     struct PreorderIndex {}
26 | | }
   | | ^
   | | |
   | |_expected `u32`, found pattern `u32 is ..=4294967040`
   |
  ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
   |
   |
53 |   pub fn newtype_index(input: TokenStream) -> TokenStream {
   |
   = note:      expected type `u32`
   = note:      expected type `u32`
           found pattern type `u32 is ..=4294967040`
error[E0308]: mismatched types
  --> compiler/rustc_data_structures/src/graph/dominators/mod.rs:24:1
   |
24 | / rustc_index::newtype_index! {
24 | / rustc_index::newtype_index! {
25 | |     struct PreorderIndex {}
26 | | }
   | | ^
   | | |
   | | expected `u32`, found pattern `u32 is ..=4294967040`
   | |_arguments to this method are incorrect
   |
  ::: /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_macros/src/lib.rs:53:1
   |
   |
53 |   pub fn newtype_index(input: TokenStream) -> TokenStream {
   |
   = note:      expected type `u32`
   = note:      expected type `u32`
           found pattern type `u32 is ..=4294967040`
note: associated function defined here
  --> /rustc/bfa3ec2e98602555e614e97839ef4ac24b82bf84/compiler/rustc_serialize/src/serialize.rs:30:8
   |
30 |     fn emit_u32(&mut self, v: u32);

For more information about this error, try `rustc --explain E0308`.
[RUSTC-TIMING] rustc_data_structures test:false 1.189
error: could not compile `rustc_data_structures` due to 8 previous errors
