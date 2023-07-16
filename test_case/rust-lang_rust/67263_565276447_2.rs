\n"},"level":"error","spans":[{"file_name":"tests/ui/infallible_destructuring_match.rs","byte_start":1155,"byte_end":1156,"line_start":57,"line_end":57,"column_start":30,"column_end":31,"is_primary":true,"text":[{"text":"    let wrapper: Result<i32, !> = Ok(23);","highlight_start":30,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/ui/infallible_destructuring_match.rs:57:30\n   |\nLL |     let wrapper: Result<i32, !> = Ok(23);\n   |                              ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T01:58:40.6302706Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T01:58:40.6302846Z 
2019-12-13T01:58:40.6303123Z ------------------------------------------
2019-12-13T01:58:40.6303178Z 
---
2019-12-13T01:59:02.8181855Z normalized stderr:
2019-12-13T01:59:02.8182064Z error[E0658]: The `!` type is experimental
2019-12-13T01:59:02.8182851Z   --> $DIR/must_use_candidates.rs:51:31
2019-12-13T01:59:02.8182953Z    |
2019-12-13T01:59:02.8183214Z LL | pub fn quoth_the_raven(_more: !) -> u32 {
2019-12-13T01:59:02.8183437Z    |
2019-12-13T01:59:02.8183738Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:02.8183738Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:02.8183864Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T01:59:02.8184025Z error: aborting due to previous error
2019-12-13T01:59:02.8184074Z 
2019-12-13T01:59:02.8184348Z For more information about this error, try `rustc --explain E0658`.
2019-12-13T01:59:02.8184425Z 
2019-12-13T01:59:02.8184425Z 
2019-12-13T01:59:02.8184462Z 
2019-12-13T01:59:02.8184546Z expected stderr:
2019-12-13T01:59:02.8191074Z error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8192005Z   --> $DIR/must_use_candidates.rs:11:1
2019-12-13T01:59:02.8192535Z    |
2019-12-13T01:59:02.8193075Z LL | pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8193470Z    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`
2019-12-13T01:59:02.8193904Z    = note: `-D clippy::must-use-candidate` implied by `-D warnings`
2019-12-13T01:59:02.8193984Z 
2019-12-13T01:59:02.8194059Z error: this method could have a `#[must_use]` attribute
2019-12-13T01:59:02.8194412Z   --> $DIR/must_use_candidates.rs:16:5
2019-12-13T01:59:02.8194412Z   --> $DIR/must_use_candidates.rs:16:5
2019-12-13T01:59:02.8194490Z    |
2019-12-13T01:59:02.8195016Z LL |     pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8195359Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`
2019-12-13T01:59:02.8195694Z error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8196116Z   --> $DIR/must_use_candidates.rs:47:1
2019-12-13T01:59:02.8196202Z    |
2019-12-13T01:59:02.8196202Z    |
2019-12-13T01:59:02.8196461Z LL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8196989Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`
2019-12-13T01:59:02.8197179Z error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8197441Z   --> $DIR/must_use_candidates.rs:59:1
2019-12-13T01:59:02.8197527Z    |
2019-12-13T01:59:02.8197527Z    |
2019-12-13T01:59:02.8197741Z LL | pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8198077Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`
2019-12-13T01:59:02.8198229Z error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8198458Z   --> $DIR/must_use_candidates.rs:67:1
2019-12-13T01:59:02.8198544Z    |
2019-12-13T01:59:02.8198544Z    |
2019-12-13T01:59:02.8198761Z LL | pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8199095Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`
2019-12-13T01:59:02.8199240Z error: aborting due to 5 previous errors
2019-12-13T01:59:02.8199294Z 
2019-12-13T01:59:02.8202950Z 
2019-12-13T01:59:02.8204873Z 
2019-12-13T01:59:02.8204873Z 
2019-12-13T01:59:02.8207885Z diff of stderr:
2019-12-13T01:59:02.8235853Z 
2019-12-13T01:59:02.8242484Z -error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8242915Z -  --> $DIR/must_use_candidates.rs:11:1
2019-12-13T01:59:02.8243002Z +error[E0658]: The `!` type is experimental
2019-12-13T01:59:02.8243293Z +  --> $DIR/must_use_candidates.rs:51:31
2019-12-13T01:59:02.8243386Z     |
2019-12-13T01:59:02.8243614Z -LL | pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8243943Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn pure(i: u8) -> u8`
2019-12-13T01:59:02.8244216Z +LL | pub fn quoth_the_raven(_more: !) -> u32 {
2019-12-13T01:59:02.8244384Z     |
2019-12-13T01:59:02.8244671Z -   = note: `-D clippy::must-use-candidate` implied by `-D warnings`
2019-12-13T01:59:02.8244988Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:02.8244988Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:02.8245118Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T01:59:02.8245479Z -error: this method could have a `#[must_use]` attribute
2019-12-13T01:59:02.8245743Z -  --> $DIR/must_use_candidates.rs:16:5
2019-12-13T01:59:02.8245961Z -   |
2019-12-13T01:59:02.8245961Z -   |
2019-12-13T01:59:02.8246220Z -LL |     pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8246561Z -   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn inherent_pure(&self) -> u8`
2019-12-13T01:59:02.8246782Z  
2019-12-13T01:59:02.8247200Z -error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8247455Z -  --> $DIR/must_use_candidates.rs:47:1
2019-12-13T01:59:02.8247651Z -   |
2019-12-13T01:59:02.8247651Z -   |
2019-12-13T01:59:02.8247935Z -LL | pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8248357Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool`
2019-12-13T01:59:02.8248881Z -error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8249282Z -  --> $DIR/must_use_candidates.rs:59:1
2019-12-13T01:59:02.8249718Z -   |
2019-12-13T01:59:02.8249718Z -   |
2019-12-13T01:59:02.8249932Z -LL | pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8250260Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn rcd(_x: Rc<u32>) -> bool`
2019-12-13T01:59:02.8250718Z -error: this function could have a `#[must_use]` attribute
2019-12-13T01:59:02.8250963Z -  --> $DIR/must_use_candidates.rs:67:1
2019-12-13T01:59:02.8251152Z -   |
2019-12-13T01:59:02.8251152Z -   |
2019-12-13T01:59:02.8251384Z -LL | pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8252236Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: add the attribute: `#[must_use] pub fn arcd(_x: Arc<u32>) -> bool`
2019-12-13T01:59:02.8252769Z -error: aborting due to 5 previous errors
2019-12-13T01:59:02.8252988Z -
2019-12-13T01:59:02.8253273Z +For more information about this error, try `rustc --explain E0658`.
2019-12-13T01:59:02.8253356Z  
---
2019-12-13T01:59:02.8254563Z use std::rc::Rc;
2019-12-13T01:59:02.8254656Z use std::sync::atomic::{AtomicBool, Ordering};
2019-12-13T01:59:02.8254731Z use std::sync::Arc;
2019-12-13T01:59:02.8254798Z 
2019-12-13T01:59:02.8254871Z pub struct MyAtomic(AtomicBool);
2019-12-13T01:59:02.8254962Z pub struct MyPure;
2019-12-13T01:59:02.8255005Z 
2019-12-13T01:59:02.8255247Z pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8255399Z }
2019-12-13T01:59:02.8255436Z 
2019-12-13T01:59:02.8255609Z impl MyPure {
2019-12-13T01:59:02.8255609Z impl MyPure {
2019-12-13T01:59:02.8255854Z     pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8256008Z     }
2019-12-13T01:59:02.8256086Z }
2019-12-13T01:59:02.8256122Z 
2019-12-13T01:59:02.8256122Z 
2019-12-13T01:59:02.8256203Z pub trait MyPureTrait {
2019-12-13T01:59:02.8256450Z     fn trait_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8256545Z         self.trait_impl_pure(i) + 1
2019-12-13T01:59:02.8256669Z 
2019-12-13T01:59:02.8256669Z 
2019-12-13T01:59:02.8256960Z     fn trait_impl_pure(&self, i: u32) -> u32;
2019-12-13T01:59:02.8257090Z 
2019-12-13T01:59:02.8257090Z 
2019-12-13T01:59:02.8257172Z impl MyPureTrait for MyPure {
2019-12-13T01:59:02.8257595Z     fn trait_impl_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8257907Z     }
2019-12-13T01:59:02.8257979Z }
2019-12-13T01:59:02.8258013Z 
2019-12-13T01:59:02.8258088Z pub fn without_result() {
2019-12-13T01:59:02.8258088Z pub fn without_result() {
2019-12-13T01:59:02.8258152Z     // OK
2019-12-13T01:59:02.8258226Z }
2019-12-13T01:59:02.8258260Z 
2019-12-13T01:59:02.8258487Z pub fn impure_primitive(i: &mut u8) -> u8 {
2019-12-13T01:59:02.8258583Z     *i
2019-12-13T01:59:02.8258691Z 
2019-12-13T01:59:02.8258691Z 
2019-12-13T01:59:02.8258931Z pub fn with_callback<F: Fn(u32) -> bool>(f: &F) -> bool {
2019-12-13T01:59:02.8259081Z }
2019-12-13T01:59:02.8259132Z 
2019-12-13T01:59:02.8259132Z 
2019-12-13T01:59:02.8259382Z pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8259534Z }
2019-12-13T01:59:02.8259587Z 
2019-12-13T01:59:02.8259587Z 
2019-12-13T01:59:02.8259803Z pub fn quoth_the_raven(_more: !) -> u32 {
2019-12-13T01:59:02.8259954Z }
2019-12-13T01:59:02.8260014Z 
2019-12-13T01:59:02.8260014Z 
2019-12-13T01:59:02.8260229Z pub fn atomics(b: &AtomicBool) -> bool {
2019-12-13T01:59:02.8260317Z     b.load(Ordering::SeqCst)
2019-12-13T01:59:02.8260433Z 
2019-12-13T01:59:02.8260433Z 
2019-12-13T01:59:02.8260643Z pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8260785Z }
2019-12-13T01:59:02.8260916Z 
2019-12-13T01:59:02.8260916Z 
2019-12-13T01:59:02.8261178Z pub fn rcmut(_x: Rc<&mut u32>) -> bool {
2019-12-13T01:59:02.8261495Z }
2019-12-13T01:59:02.8261727Z 
2019-12-13T01:59:02.8261727Z 
2019-12-13T01:59:02.8262009Z pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8262164Z }
2019-12-13T01:59:02.8262200Z 
2019-12-13T01:59:02.8262200Z 
2019-12-13T01:59:02.8262455Z pub fn inner_types(_m: &MyAtomic) -> bool {
2019-12-13T01:59:02.8262609Z }
2019-12-13T01:59:02.8262645Z 
2019-12-13T01:59:02.8262725Z static mut COUNTER: usize = 0;
2019-12-13T01:59:02.8262770Z 
---
2019-12-13T01:59:02.8263729Z     COUNTER
2019-12-13T01:59:02.8263816Z }
2019-12-13T01:59:02.8263853Z 
2019-12-13T01:59:02.8263943Z fn main() {
2019-12-13T01:59:02.8264008Z     assert_eq!(1, pure(1));
2019-12-13T01:59:02.8264127Z 
2019-12-13T01:59:02.8366443Z 
2019-12-13T01:59:02.8366609Z expected fixed:
2019-12-13T01:59:02.8367146Z // run-rustfix
2019-12-13T01:59:02.8367146Z // run-rustfix
2019-12-13T01:59:02.8367223Z #![allow(unused_mut)]
2019-12-13T01:59:02.8367312Z #![warn(clippy::must_use_candidate)]
2019-12-13T01:59:02.8367385Z use std::rc::Rc;
2019-12-13T01:59:02.8367476Z use std::sync::atomic::{AtomicBool, Ordering};
2019-12-13T01:59:02.8367552Z use std::sync::Arc;
2019-12-13T01:59:02.8367613Z 
2019-12-13T01:59:02.8367677Z pub struct MyAtomic(AtomicBool);
2019-12-13T01:59:02.8367787Z pub struct MyPure;
2019-12-13T01:59:02.8367830Z 
2019-12-13T01:59:02.8368107Z #[must_use] pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8368256Z }
2019-12-13T01:59:02.8368293Z 
2019-12-13T01:59:02.8368366Z impl MyPure {
2019-12-13T01:59:02.8368366Z impl MyPure {
2019-12-13T01:59:02.8368628Z     #[must_use] pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8368794Z     }
2019-12-13T01:59:02.8368871Z }
2019-12-13T01:59:02.8368907Z 
2019-12-13T01:59:02.8368907Z 
2019-12-13T01:59:02.8368986Z pub trait MyPureTrait {
2019-12-13T01:59:02.8369238Z     fn trait_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8369332Z         self.trait_impl_pure(i) + 1
2019-12-13T01:59:02.8369456Z 
2019-12-13T01:59:02.8369456Z 
2019-12-13T01:59:02.8369864Z     fn trait_impl_pure(&self, i: u32) -> u32;
2019-12-13T01:59:02.8369987Z 
2019-12-13T01:59:02.8369987Z 
2019-12-13T01:59:02.8370063Z impl MyPureTrait for MyPure {
2019-12-13T01:59:02.8370312Z     fn trait_impl_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8370471Z     }
2019-12-13T01:59:02.8370543Z }
2019-12-13T01:59:02.8370578Z 
2019-12-13T01:59:02.8370654Z pub fn without_result() {
2019-12-13T01:59:02.8370654Z pub fn without_result() {
2019-12-13T01:59:02.8370721Z     // OK
2019-12-13T01:59:02.8370796Z }
2019-12-13T01:59:02.8370831Z 
2019-12-13T01:59:02.8371231Z pub fn impure_primitive(i: &mut u8) -> u8 {
2019-12-13T01:59:02.8371696Z     *i
2019-12-13T01:59:02.8371808Z 
2019-12-13T01:59:02.8371808Z 
2019-12-13T01:59:02.8372112Z pub fn with_callback<F: Fn(u32) -> bool>(f: &F) -> bool {
2019-12-13T01:59:02.8372273Z }
2019-12-13T01:59:02.8372327Z 
2019-12-13T01:59:02.8372327Z 
2019-12-13T01:59:02.8372617Z #[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8372792Z }
2019-12-13T01:59:02.8372843Z 
2019-12-13T01:59:02.8372843Z 
2019-12-13T01:59:02.8373079Z pub fn quoth_the_raven(_more: !) -> u32 {
2019-12-13T01:59:02.8373238Z }
2019-12-13T01:59:02.8373289Z 
2019-12-13T01:59:02.8373289Z 
2019-12-13T01:59:02.8373533Z pub fn atomics(b: &AtomicBool) -> bool {
2019-12-13T01:59:02.8373629Z     b.load(Ordering::SeqCst)
2019-12-13T01:59:02.8373748Z 
2019-12-13T01:59:02.8373748Z 
2019-12-13T01:59:02.8373993Z #[must_use] pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8374146Z }
2019-12-13T01:59:02.8374197Z 
2019-12-13T01:59:02.8374197Z 
2019-12-13T01:59:02.8374698Z pub fn rcmut(_x: Rc<&mut u32>) -> bool {
2019-12-13T01:59:02.8374850Z }
2019-12-13T01:59:02.8374887Z 
2019-12-13T01:59:02.8374887Z 
2019-12-13T01:59:02.8375148Z #[must_use] pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8375301Z }
2019-12-13T01:59:02.8375338Z 
2019-12-13T01:59:02.8375338Z 
2019-12-13T01:59:02.8375592Z pub fn inner_types(_m: &MyAtomic) -> bool {
2019-12-13T01:59:02.8375746Z }
2019-12-13T01:59:02.8375782Z 
2019-12-13T01:59:02.8375862Z static mut COUNTER: usize = 0;
2019-12-13T01:59:02.8375908Z 
---
2019-12-13T01:59:02.8376868Z     COUNTER
2019-12-13T01:59:02.8376947Z }
2019-12-13T01:59:02.8376983Z 
2019-12-13T01:59:02.8377060Z fn main() {
2019-12-13T01:59:02.8377136Z     assert_eq!(1, pure(1));
2019-12-13T01:59:02.8377255Z 
2019-12-13T01:59:02.8377292Z 
2019-12-13T01:59:02.8377370Z diff of fixed:
2019-12-13T01:59:02.8377411Z 
2019-12-13T01:59:02.8377411Z 
2019-12-13T01:59:02.8377646Z  // run-rustfix
2019-12-13T01:59:02.8377716Z  #![allow(unused_mut)]
2019-12-13T01:59:02.8377804Z  #![warn(clippy::must_use_candidate)]
2019-12-13T01:59:02.8378040Z  use std::rc::Rc;
2019-12-13T01:59:02.8378135Z  use std::sync::atomic::{AtomicBool, Ordering};
2019-12-13T01:59:02.8378223Z  use std::sync::Arc;
2019-12-13T01:59:02.8378286Z  
2019-12-13T01:59:02.8378360Z  pub struct MyAtomic(AtomicBool);
2019-12-13T01:59:02.8378439Z  pub struct MyPure;
2019-12-13T01:59:02.8378517Z  
2019-12-13T01:59:02.8378750Z -#[must_use] pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8378992Z +pub fn pure(i: u8) -> u8 {
2019-12-13T01:59:02.8379299Z  }
2019-12-13T01:59:02.8379355Z  
2019-12-13T01:59:02.8379355Z  
2019-12-13T01:59:02.8379430Z  impl MyPure {
2019-12-13T01:59:02.8379876Z -    #[must_use] pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8380130Z +    pub fn inherent_pure(&self) -> u8 {
2019-12-13T01:59:02.8380278Z      }
2019-12-13T01:59:02.8380356Z  }
2019-12-13T01:59:02.8380413Z  
2019-12-13T01:59:02.8380413Z  
2019-12-13T01:59:02.8380488Z  pub trait MyPureTrait {
2019-12-13T01:59:02.8380725Z      fn trait_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8380818Z          self.trait_impl_pure(i) + 1
2019-12-13T01:59:02.8380957Z  
2019-12-13T01:59:02.8380957Z  
2019-12-13T01:59:02.8381214Z      fn trait_impl_pure(&self, i: u32) -> u32;
2019-12-13T01:59:02.8381358Z  
2019-12-13T01:59:02.8381358Z  
2019-12-13T01:59:02.8381426Z  impl MyPureTrait for MyPure {
2019-12-13T01:59:02.8382336Z      fn trait_impl_pure(&self, i: u32) -> u32 {
2019-12-13T01:59:02.8382498Z      }
2019-12-13T01:59:02.8382558Z  }
2019-12-13T01:59:02.8382633Z  
2019-12-13T01:59:02.8382695Z  pub fn without_result() {
2019-12-13T01:59:02.8382695Z  pub fn without_result() {
2019-12-13T01:59:02.8382780Z      // OK
2019-12-13T01:59:02.8382852Z  }
2019-12-13T01:59:02.8382927Z  
2019-12-13T01:59:02.8383212Z  pub fn impure_primitive(i: &mut u8) -> u8 {
2019-12-13T01:59:02.8383290Z      *i
2019-12-13T01:59:02.8383423Z  
2019-12-13T01:59:02.8383423Z  
2019-12-13T01:59:02.8383698Z  pub fn with_callback<F: Fn(u32) -> bool>(f: &F) -> bool {
2019-12-13T01:59:02.8383855Z  }
2019-12-13T01:59:02.8383915Z  
2019-12-13T01:59:02.8383915Z  
2019-12-13T01:59:02.8384218Z -#[must_use] pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8384522Z +pub fn with_marker(_d: std::marker::PhantomData<&mut u32>) -> bool {
2019-12-13T01:59:02.8384706Z  }
2019-12-13T01:59:02.8384766Z  
2019-12-13T01:59:02.8384766Z  
2019-12-13T01:59:02.8385023Z  pub fn quoth_the_raven(_more: !) -> u32 {
2019-12-13T01:59:02.8385182Z  }
2019-12-13T01:59:02.8385241Z  
2019-12-13T01:59:02.8385241Z  
2019-12-13T01:59:02.8385491Z  pub fn atomics(b: &AtomicBool) -> bool {
2019-12-13T01:59:02.8385689Z      b.load(Ordering::SeqCst)
2019-12-13T01:59:02.8385835Z  
2019-12-13T01:59:02.8385835Z  
2019-12-13T01:59:02.8386126Z -#[must_use] pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8386381Z +pub fn rcd(_x: Rc<u32>) -> bool {
2019-12-13T01:59:02.8386533Z  }
2019-12-13T01:59:02.8386593Z  
2019-12-13T01:59:02.8386593Z  
2019-12-13T01:59:02.8386845Z  pub fn rcmut(_x: Rc<&mut u32>) -> bool {
2019-12-13T01:59:02.8386997Z  }
2019-12-13T01:59:02.8387056Z  
2019-12-13T01:59:02.8387056Z  
2019-12-13T01:59:02.8387320Z -#[must_use] pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8387561Z +pub fn arcd(_x: Arc<u32>) -> bool {
2019-12-13T01:59:02.8387807Z  }
2019-12-13T01:59:02.8387883Z  
2019-12-13T01:59:02.8387883Z  
2019-12-13T01:59:02.8388166Z  pub fn inner_types(_m: &MyAtomic) -> bool {
2019-12-13T01:59:02.8388323Z  }
2019-12-13T01:59:02.8388383Z  
2019-12-13T01:59:02.8388464Z  static mut COUNTER: usize = 0;
2019-12-13T01:59:02.8388533Z  
---
2019-12-13T01:59:02.8389374Z      COUNTER
2019-12-13T01:59:02.8389438Z  }
2019-12-13T01:59:02.8389513Z  
2019-12-13T01:59:02.8389574Z  fn main() {
2019-12-13T01:59:02.8389655Z      assert_eq!(1, pure(1));
2019-12-13T01:59:02.8389798Z  
2019-12-13T01:59:02.8389835Z 
2019-12-13T01:59:02.8389920Z The actual fixed differed from the expected fixed.
2019-12-13T01:59:02.8389920Z The actual fixed differed from the expected fixed.
2019-12-13T01:59:02.8390383Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/must_use_candidates.fixed
2019-12-13T01:59:02.8391006Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base' 'must_use_candidates.rs'
2019-12-13T01:59:02.8391132Z 
2019-12-13T01:59:02.8391199Z error: 2 errors occurred comparing output.
2019-12-13T01:59:02.8391288Z status: exit code: 1
2019-12-13T01:59:02.8391288Z status: exit code: 1
2019-12-13T01:59:02.8394124Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/must_use_candidates.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/must_use_candidates.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-70d9308013a308bf.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4cf2006b2fb0099f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-93610b460d310c52.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-317c1b266af760ab/out/test_build_base/must_use_candidates.stage-id.aux" "-A" "unused"
2019-12-13T01:59:02.8395190Z ------------------------------------------
2019-12-13T01:59:02.8395362Z 
2019-12-13T01:59:02.8395745Z ------------------------------------------
2019-12-13T01:59:02.8395839Z stderr:
2019-12-13T01:59:02.8395839Z stderr:
2019-12-13T01:59:02.8396075Z ------------------------------------------
2019-12-13T01:59:02.8398724Z {"message":"The `!` type is experimental","code":{"code":"E0658","explanation":"An unstable feature was used.\n\nErroneous code example:\n\n