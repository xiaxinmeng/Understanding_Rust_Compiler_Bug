plain
travis_time:end:051648c6:start=1544021123821145053,finish=1544021124871997896,duration=1050852843
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:50]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:02:56]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:02:56]    Compiling cmake v0.1.33
[00:02:56]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:123:6
[00:02:56]     |
[00:02:56] 123 | impl fmt::Debug for dyn Any {
[00:02:56]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:133:6
[00:02:56]     |
[00:02:56] 133 | impl fmt::Debug for dyn Any + Send {
[00:02:56]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:140:6
[00:02:56]     |
[00:02:56] 140 | impl fmt::Debug for dyn Any + Send + Sync {
[00:02:56]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:146:6
[00:02:56]     |
[00:02:56] 146 | impl dyn Any {
[00:02:56]     |      ^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:251:6
[00:02:56]     |
[00:02:56] 251 | impl dyn Any+Send {
[00:02:56]     |      ^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:56]    --> src/libcore/any.rs:335:6
[00:02:56]     |
[00:02:56] 335 | impl dyn Any+Send+Sync {
[00:02:56]     |      ^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `iter::iterator::Iterator` cannot be made into an object
[00:02:56]    |
[00:02:56]    |
[00:02:56] 21 | fn _assert_is_object_safe(_: &dyn Iterator<Item=()>) {}
[00:02:56]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `iter::iterator::Iterator` cannot be made into an object
[00:02:56]    |
[00:02:56]    = note: method `next`'s receiver cannot be dispatched on
[00:02:56]    = note: method `size_hint`'s receiver cannot be dispatched on
[00:02:56]    = note: method `nth`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:02:56]   --> src/libcore/fmt/builders.rs:14:5
[00:02:56]    |
[00:02:56] 14 |     buf: &'a mut (dyn fmt::Write + 'a),
[00:02:56]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:02:56]    |
[00:02:56]    = note: method `write_str`'s receiver cannot be dispatched on
[00:02:56]    = note: method `write_char`'s receiver cannot be dispatched on
[00:02:56]    = note: method `write_fmt`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:02:56]    --> src/libcore/fmt/mod.rs:257:5
[00:02:56]     |
[00:02:56] 257 |     buf: &'a mut (dyn Write+'a),
[00:02:56]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `write_str`'s receiver cannot be dispatched on
[00:02:56]     = note: method `write_char`'s receiver cannot be dispatched on
[00:02:56]     = note: method `write_fmt`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `ops::function::Fn` cannot be made into an object
[00:02:56]    --> src/libcore/fmt/mod.rs:274:5
[00:02:56]     |
[00:02:56] 274 |     _oibit_remover: PhantomData<*mut dyn Fn()>,
[00:02:56]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ops::function::Fn` cannot be made into an object
[00:02:56]     |
[00:02:56]     = note: method `call`'s receiver cannot be dispatched on
[00:02:56]     = note: method `call_mut`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:02:56]     --> src/libcore/fmt/mod.rs:1022:1
[00:02:56]      |
[00:02:56] 1022 | pub fn write(output: &mut dyn Write, args: Arguments) -> Result {
[00:02:56]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:02:56]      |
[00:02:56]      = note: method `write_str`'s receiver cannot be dispatched on
[00:02:56]      = note: method `write_char`'s receiver cannot be dispatched on
[00:02:56]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:02:56]     --> src/libcore/fmt/mod.rs:1374:9
[00:02:56]      |
[00:02:56] 1374 |         fn write_bytes(buf: &mut dyn Write, s: &[u8]) -> Result {
[00:02:56]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:02:56]      |
[00:02:56]      = note: method `write_str`'s receiver cannot be dispatched on
[00:02:56]      = note: method `write_char`'s receiver cannot be dispatched on
[00:02:56]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:02:56] 
[00:02:56] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:56]   --> src/libcore/task/wake.rs:26:5
[00:02:56]    |
[00:02:56] 26 |     inner: NonNull<dyn UnsafeWake>,
[00:02:56]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:56]    |
[00:02:56]    = note: method `clone_raw`'s receiver cannot be dispatched on
[00:02:56]    = note: method `drop_raw`'s receiver cannot be dispatched on
[00:02:56]    = note: method `wake`'s receiver cannot be dispatched on
[00:02:56]    = note: method `wake_local`'s receiver cannot be dispatched on
12m--> src/libcore/panic.rs:274:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 274 |     fn get(&mut self) -> &(dyn Any + Send);
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:124:5
[00:02:57] 124 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57] 124 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:134:5
[00:02:57] 134 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57] 134 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:141:5
[00:02:57] 141 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57] 141 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:169:5
[00:02:57]     |
[00:02:57] 169 |     pub fn is<T: Any>(&self) -> bool {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:203:5
[00:02:57]     |
[00:02:57] 203 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:240:5
[00:02:57]     |
[00:02:57] 240 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:274:5
[00:02:57]     |
[00:02:57] 274 |     pub fn is<T: Any>(&self) -> bool {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:300:5
[00:02:57]     |
[00:02:57] 300 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:330:5
[00:02:57]     |
[00:02:57] 330 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:358:5
[00:02:57]     |
[00:02:57] 358 |     pub fn is<T: Any>(&self) -> bool {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:384:5
[00:02:57]     |
[00:02:57] 384 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `any::Any` cannot be made into an object
[00:02:57]    --> src/libcore/any.rs:414:5
[00:02:57]     |
[00:02:57] 414 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]src/libcore/fmt/builders.rs:110:5
[00:02:57]     |
[00:02:57] 110 |     pub fn field(&mut self, name: &str, value: &dyn fmt::Debug) -> &mut DebugStruct<'a, 'b> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:02:57]    --> src/libcore/fmt/builders.rs:207:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 207 |     pub fn field(&mut self, value: &dyn fmt::Debug) -> &mut DebugTuple<'a, 'b> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:02:57]    --> src/libcore/fmt/builders.rs:261:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 261 |     fn entry(&mut self, entry: &dyn fmt::Debug) {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:02:57]    --> src/libcore/fmt/builders.rs:343:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 343 |     pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugSet<'a, 'b> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:02:57]    --> src/libcore/fmt/builders.rs:414:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 414 |     pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugList<'a, 'b> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:02:57]    --> src/libcore/fmt/builders.rs:485:5
[00:02:57]     |
[00:02:57]     |
[00:02:57] 485 |     pub fn entry(&mut self, key: &dyn fmt::Debug, value: &dyn fmt::Debug) -> &mut DebugMap<'a, 'b> {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `fmt`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:02:57]     --> src/libcore/fmt/mod.rs:1063:5
[00:02:57]      |
[00:02:57] 1063 | /     fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
[00:02:57] 1064 | |         where 'b: 'c, F: FnOnce(&'b mut (dyn Write+'b)) -> &'c mut (dyn Write+'c)
[00:02:57] 1066 | |         Formatter {
[00:02:57] ...    |
[00:02:57] 1081 | |         }
[00:02:57] 1082 | |     }
[00:02:57] 1082 | |     }
[00:02:57]      | |_____^ the trait `fmt::Write` cannot be made into an object
[00:02:57]      |
[00:02:57]      = note: method `write_str`'s receiver cannot be dispatched on
[00:02:57]      = note: method `write_char`'s receiver cannot be dispatched on
[00:02:57]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:57]   --> src/libcore/task/wake.rs:44:5
[00:02:57]    |
[00:02:57] 44 |     pub unsafe fn new(inner: NonNull<dyn UnsafeWake>) -> Self {
[00:02:57]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:57]    |
[00:02:57]    = note: method `clone_raw`'s receiver cannot be dispatched on
[00:02:57]    = note: method `drop_raw`'s receiver cannot be dispatched on
[00:02:57]    = note: method `wake`'s receiver cannot be dispatched on
[00:02:57]    = note: method `wake_local`'s receiver cannot be dispatched on
[00:02:57] 
[00:02:57] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:57]    --> src/libcore/task/wake.rs:135:5
[00:02:57]     |
[00:02:57] 135 |     pub unsafe fn new(inner: NonNull<dyn UnsafeWake>) -> Self {
[00:02:57]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:02:57]     |
[00:02:57]     = note: method `clone_raw`'s receiver cannot be dispatched on
[00:02:57]     = note: method `drop_raw`'s receiver cannot be dispatched on
[00:02:57]     = note: method `wake`'s receiver cannot be dispatched on
[00:02:57]     = note: method `wake_local`'s receiver cannot be dispatched on
