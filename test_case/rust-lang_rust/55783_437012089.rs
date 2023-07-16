plain
travis_time:end:098878d4:start=1541686039113725837,finish=1541686096391108453,duration=57277382616
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:20:05]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:20:06]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:20:06]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:20:11]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 286 | pub use self::select::{Select, Handle};
[00:20:13]     |
[00:20:13]     = note: `-D deprecated` implied by `-D warnings`
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 286 | pub use self::select::{Select, Handle};
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 287 | use self::select::StartResult;
[00:20:13]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 288 | use self::select::StartResult::*;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::SelectInner': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 73 |     inner: UnsafeCell<SelectInner>,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 78 |     head: *mut Handle<'static, ()>,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 79 |     tail: *mut Handle<'static, ()>,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 82 | impl !marker::Send for Select {}
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::SelectInner': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13] 91 |     selector: *mut SelectInner,
[00:20:13]    |                    ^^^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 92 |     next: *mut Handle<'static, ()>,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 93 |     prev: *mut Handle<'static, ()>,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Packet': channel selection will be removed in a future release
[00:20:13]    |
[00:20:13]    |
[00:20:13] 95 |     packet: &'rx (dyn Packet+'rx),
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 102 | struct Packets { cur: *mut Handle<'static, ()> }
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 118 | impl Select {
[00:20:13]     |      ^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 263 | impl<'rx, T: Send> Handle<'rx, T> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 325 | impl Drop for Select {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 334 | impl<'rx, T: Send> Drop for Handle<'rx, T> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Packets': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 340 | impl Iterator for Packets {
[00:20:13]     |                   ^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 354 | impl fmt::Debug for Select {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 360 | impl<'rx, T:Send+'rx> fmt::Debug for Handle<'rx, T> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]   --> libstd/sync/mpsc/shared.rs:32:5
[00:20:13] 32 | use sync::mpsc::select::StartResult::*;
[00:20:13]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]   --> libstd/sync/mpsc/shared.rs:33:5
[00:20:13] 33 | use sync::mpsc::select::StartResult;
[00:20:13]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]   --> libstd/sync/mpsc/sync.rs:46:39
[00:20:13]    |
[00:20:13] 46 | use sync::mpsc::select::StartResult::{self, Installed, Abort};
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]   --> libstd/sync/mpsc/sync.rs:46:45
[00:20:13]    |
[00:20:13] 46 | use sync::mpsc::select::StartResult::{self, Installed, Abort};
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]   --> libstd/sync/mpsc/sync.rs:46:56
[00:20:13]    |
[00:20:13] 46 | use sync::mpsc::select::StartResult::{self, Installed, Abort};
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Packet': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1520 | impl<T> select::Packet for Receiver<T> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 114 |     fn start_selection(&self, token: SignalToken) -> StartResult;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 133 |     pub fn new() -> Select {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Select': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 134 |         Select {
[00:20:13]     |         ^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::SelectInner': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 135 |             inner: UnsafeCell::new(SelectInner {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 146 |     pub fn handle<'a, T: Send>(&'a self, rx: &'a Receiver<T>) -> Handle<'a, T> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 149 |         Handle {
[00:20:13]     |         ^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13] 210 |                     StartResult::Installed => {}
[00:20:13]     |                     ^^^^^^^^^^^^^^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 211 |                     StartResult::Abort => {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Packets': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 260 |     fn iter(&self) -> Packets { Packets { cur: unsafe { &*self.inner.get() }.head } }
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Packets': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 260 |     fn iter(&self) -> Packets { Packets { cur: unsafe { &*self.inner.get() }.head } }
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 282 |         let me = self as *mut Handle<'rx, T> as *mut Handle<'static, ()>;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 282 |         let me = self as *mut Handle<'rx, T> as *mut Handle<'static, ()>;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 303 |         let me = self as *mut Handle<'rx, T> as *mut Handle<'static, ()>;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 303 |         let me = self as *mut Handle<'rx, T> as *mut Handle<'static, ()>;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 341 |     type Item = *mut Handle<'static, ()>;
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::Handle': channel selection will be removed in a future release
[00:20:13]     |
[00:20:13]     |
[00:20:13] 343 |     fn next(&mut self) -> Option<*mut Handle<'static, ()>> {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:232:44
[00:20:13]     |
[00:20:13] 232 |         if self.decrement(signal_token) == Installed {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:251:48
[00:20:13]     |
[00:20:13] 251 |     fn decrement(&self, token: SignalToken) -> StartResult {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:265:49
[00:20:13]     |
[00:20:13] 265 |                     if n - steals <= 0 { return Installed }
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:271:13
[00:20:13] 271 |             Abort
[00:20:13]     |             ^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:433:58
[00:20:13]     |
[00:20:13] 433 |     pub fn start_selection(&self, token: SignalToken) -> StartResult {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:435:13
[00:20:13] 435 |             Installed => Installed,
[00:20:13]     |             ^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:435:26
[00:20:13] 435 |             Installed => Installed,
[00:20:13]     |                          ^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:436:13
[00:20:13] 436 |             Abort => {
[00:20:13]     |             ^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/shared.rs:439:17
[00:20:13] 439 |                 Abort
[00:20:13]     |                 ^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/sync.rs:433:58
[00:20:13]     |
[00:20:13] 433 |     pub fn start_selection(&self, token: SignalToken) -> StartResult {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/sync.rs:436:13
[00:20:13] 436 |             Abort
[00:20:13]     |             ^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]    --> libstd/sync/mpsc/sync.rs:443:13
[00:20:13] 443 |             Installed
[00:20:13]     |             ^^^^^^^^^
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1546 |     fn start_selection(&self, mut token: SignalToken) -> StartResult {
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1551 |                         oneshot::SelSuccess => return Installed,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1552 |                         oneshot::SelCanceled => return Abort,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Installed': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1558 |                         stream::SelSuccess => return Installed,
[00:20:13] 
[00:20:13] 
[00:20:13] error: use of deprecated item 'sync::mpsc::select::StartResult::Abort': channel selection will be removed in a future release
[00:20:13]      |
[00:20:13]      |
[00:20:13] 1559 |                         stream::SelCanceled => return Abort,
[00:20:13] 
el selection will be removed in a future release
[00:20:15]    --> libstd/sync/mpsc/select.rs:137:17
[00:20:15]     |
[00:20:15]     |
[00:20:15] 137 |                 tail: ptr::null_mut(),
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::next_id': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 139 |             next_id: Cell::new(1),
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::next_id': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 147 |         let id = self.next_id.get();
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::next_id': channel selection witd/sync/mpsc/select.rs:154:13
[00:20:15] 154 |             added: false,
[00:20:15]     |             ^^^^^^^^^^^^
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::rx': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15] 155 |             rx,
[00:20:15]     |             ^^
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::packet': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15] 156 |             packet: rx,
[00:20:15]     |             ^^^^^^^^^^
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::wait2': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 167 |         self.wait2(true)
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::iter': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 195 |                 for handle in self.iter() {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::packet': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 196 |                     if (*handle).packet.can_recv() {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Packet::can_recv': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15] 196|
[00:20:15] 196|
[00:20:15] 209 |                 match (*handle).packet.start_selection(signal_token.clone()) {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Packet::start_selection': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 209 |                 match (*handle).packet.start_selection(signal_token.clone()) {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::iter': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 213 |                         for handle in self.iter().take(i) {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::packet': channel selection will be remove1m: use of deprecated item 'sync::mpsc::select::Select::iter': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 248 |             for handle in self.iter() {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::packet': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 249 |                 if (*handle).packet.abort_selection() {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Packet::abort_selection': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 249 |                 if (*handle).packet.abort_selection() {
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::id': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 250 |                     ready_id = (*handle).id;
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Packets::cur': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 260 |     fn iter(&self) -> Packets { Packets { cur: unsafe { &*self.inner.get() }.head } }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Select::inner': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 260 |     fn iter(&self) -> Packets { Packets { cur: unsafe { &*self.inner.get() }.head } }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::SelectInner::head': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 260 |     fn iter(&self) -> Packets { Packets { cur: unsafe { &*self.inner.get() }.head } }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::id': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 266 |     pub fn id(&self) -> usize { self.id }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::rx': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 271 |     pub fn recv(&mut self) -> Result<T, RecvError> { self.rx.recv() }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::added': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 280 |         if self.added { return }
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::Handle::selector': channel selection will be removed in a future release
[00:20:15]     |
[00:20:15]     |
[00:20:15] 281 |         let selector = &mut *self.selector;
[00:20:15] 
[00:20:15] 
[00:20:15] error: use of deprecated item 'sync::mpsc::select::SelectInner::head': channel selection will be removed in a future release
[00:20:15]     |
---
151412 ./src/tools/clang
150256 ./obj/build/bootstrap/debug/incremental
149132 ./src/llvm-emscripten/test
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6hcvr5b8j-19nko52-22tmsi8iacpi9
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
94924 ./obj/build/x86_64-unknown-linux-gnu/stage1
94904 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
travis_time:end:1390bb14:start=1541687323133653347,finish=1541687323139325252,duration=5671905
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08f95c35
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b0f1432
travis_time:start:2b0f1432
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown
