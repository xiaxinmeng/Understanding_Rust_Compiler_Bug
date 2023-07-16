plain
travis_time:end:0c8c5644:start=1542209303718178734,finish=1542209359661646917,duration=55943468183
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:19:42]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:19:42]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:19:42]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:19:43]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:123:6
[00:19:44]     |
[00:19:44] 123 | impl fmt::Debug for dyn Any {
[00:19:44]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:133:6
[00:19:44]     |
[00:19:44] 133 | impl fmt::Debug for dyn Any + Send {
[00:19:44]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:140:6
[00:19:44]     |
[00:19:44] 140 | impl fmt::Debug for dyn Any + Send + Sync {
[00:19:44]     |      ^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:146:6
[00:19:44]     |
[00:19:44] 146 | impl dyn Any {
[00:19:44]     |      ^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:251:6
[00:19:44]     |
[00:19:44] 251 | impl dyn Any+Send {
[00:19:44]     |      ^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:44]    --> libcore/any.rs:335:6
[00:19:44]     |
[00:19:44] 335 | impl dyn Any+Send+Sync {
[00:19:44]     |      ^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `iter::iterator::Iterator` cannot be made into an object
[00:19:44]    |
[00:19:44]    |
[00:19:44] 21 | fn _assert_is_object_safe(_: &dyn Iterator<Item=()>) {}
[00:19:44]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `iter::iterator::Iterator` cannot be made into an object
[00:19:44]    |
[00:19:44]    = note: method `next`'s receiver cannot be dispatched on
[00:19:44]    = note: method `size_hint`'s receiver cannot be dispatched on
[00:19:44]    = note: method `nth`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:19:44]   --> libcore/fmt/builders.rs:14:5
[00:19:44]    |
[00:19:44] 14 |     buf: &'a mut (dyn fmt::Write + 'a),
[00:19:44]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:19:44]    |
[00:19:44]    = note: method `write_str`'s receiver cannot be dispatched on
[00:19:44]    = note: method `write_char`'s receiver cannot be dispatched on
[00:19:44]    = note: method `write_fmt`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:19:44]    --> libcore/fmt/mod.rs:257:5
[00:19:44]     |
[00:19:44] 257 |     buf: &'a mut (dyn Write+'a),
[00:19:44]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `write_str`'s receiver cannot be dispatched on
[00:19:44]     = note: method `write_char`'s receiver cannot be dispatched on
[00:19:44]     = note: method `write_fmt`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `ops::function::Fn` cannot be made into an object
[00:19:44]    --> libcore/fmt/mod.rs:274:5
[00:19:44]     |
[00:19:44] 274 |     _oibit_remover: PhantomData<*mut dyn Fn()>,
[00:19:44]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ops::function::Fn` cannot be made into an object
[00:19:44]     |
[00:19:44]     = note: method `call`'s receiver cannot be dispatched on
[00:19:44]     = note: method `call_mut`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:19:44]     --> libcore/fmt/mod.rs:1022:1
[00:19:44]      |
[00:19:44] 1022 | pub fn write(output: &mut dyn Write, args: Arguments) -> Result {
[00:19:44]      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:19:44]      |
[00:19:44]      = note: method `write_str`'s receiver cannot be dispatched on
[00:19:44]      = note: method `write_char`'s receiver cannot be dispatched on
[00:19:44]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:44] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:19:44]     --> libcore/fmt/mod.rs:1374:9
[00:19:44]      |
[00:19:44] 1374 |         fn write_bytes(buf: &mut dyn Write, s: &[u8]) -> Result {
[00:19:44]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Write` cannot be made into an object
[00:19:44]      |
[00:19:44]      = note: method `write_str`'s receiver cannot be dispatched on
[00:19:44]      = note: method `write_char`'s receiver cannot be dispatched on
[00:19:44]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:19:44] 
[00:19:45] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:45]   --> libcore/task/wake.rs:26:5
[00:19:45]    |
[00:19:45] 26 |     inner: NonNull<dyn UnsafeWake>,
[00:19:45]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:45]    |
[00:19:45]    = note: method `clone_raw`'s receiver cannot be dispatched on
[00:19:45]    = note: method `drop_raw`'s receiver cannot be dispatched on
[00:19:45]    = note: method `wake`'s receiver cannot be dispatched on
[00:19:45]    = note: method `wake_local`'s receiver cannot be dispatched on
[00:19:45] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]   --> libcore/panic.rs:46:5
[00:19:46]    |
[00:19:46] 46 |     payload: &'a (dyn Any + Send),
[00:19:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]    |
[00:19:46]    = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/panic.rs:273:5
[00:19:46]     |
[00:19:46] 273 |     fn box_me_up(&mut self) -> *mut (dyn Any + Send);
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/panic.rs:274:5
[00:19:46]     |
[00:19:46] 274 |     fn get(&mut self) -> &(dyn Any + Send);
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:124:5
[00:19:46] 124 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46] 124 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:134:5
[00:19:46] 134 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46] 134 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:141:5
[00:19:46] 141 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46] 141 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:169:5
[00:19:46]     |
[00:19:46] 169 |     pub fn is<T: Any>(&self) -> bool {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:203:5
[00:19:46]     |
[00:19:46] 203 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:240:5
[00:19:46]     |
[00:19:46] 240 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:274:5
[00:19:46]     |
[00:19:46] 274 |     pub fn is<T: Any>(&self) -> bool {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:300:5
[00:19:46]     |
[00:19:46] 300 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:330:5
[00:19:46]     |
[00:19:46] 330 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:358:5
[00:19:46]     |
[00:19:46] 358 |     pub fn is<T: Any>(&self) -> bool {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:384:5
[00:19:46]     |
[00:19:46] 384 |     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]    --> libcore/any.rs:414:5
[00:19:46]     |
[00:19:46] 414 |     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]   --> libcore/panic.rs:67:5
[00:19:46]    |
[00:19:46] 67 |     pub fn set_payload(&mut self, info: &'a (dyn Any + Send)) {
[00:19:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]    |
[00:19:46]    = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `any::Any` cannot be made into an object
[00:19:46]   --> libcore/panic.rs:89:5
[00:19:46]    |
[00:19:46] 89 |     pub fn payload(&self) -> &(dyn Any + Send) {
[00:19:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `any::Any` cannot be made into an object
[00:19:46]    |
[00:19:46]    = note: method `get_type_id`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:110:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 110 |     pub fn field(&mut self, name: &str, value: &dyn fmt::Debug) -> &mut DebugStruct<'a, 'b> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:207:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 207 |     pub fn field(&mut self, value: &dyn fmt::Debug) -> &mut DebugTuple<'a, 'b> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:261:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 261 |     fn entry(&mut self, entry: &dyn fmt::Debug) {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:343:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 343 |     pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugSet<'a, 'b> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:414:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 414 |     pub fn entry(&mut self, entry: &dyn fmt::Debug) -> &mut DebugList<'a, 'b> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] error[E0038]: the trait `fmt::Debug` cannot be made into an object
[00:19:46]    --> libcore/fmt/builders.rs:485:5
[00:19:46]     |
[00:19:46]     |
[00:19:46] 485 |     pub fn entry(&mut self, key: &dyn fmt::Debug, value: &dyn fmt::Debug) -> &mut DebugMap<'a, 'b> {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `fmt::Debug` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `fmt`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `fmt::Write` cannot be made into an object
[00:19:46]     --> libcore/fmt/mod.rs:1063:5
[00:19:46]      |
[00:19:46] 1063 | /     fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
[00:19:46] 1064 | |         where 'b: 'c, F: FnOnce(&'b mut (dyn Write+'b)) -> &'c mut (dyn Write+'c)
[00:19:46] 1065 | |     {
[00:19:46] ...    |
[00:19:46] 1081 | |         }
[00:19:46] 1082 | |     }
[00:19:46] 1082 | |     }
[00:19:46]      | |_____^ the trait `fmt::Write` cannot be made into an object
[00:19:46]      |
[00:19:46]      = note: method `write_str`'s receiver cannot be dispatched on
[00:19:46]      = note: method `write_char`'s receiver cannot be dispatched on
[00:19:46]      = note: method `write_fmt`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:46]   --> libcore/task/wake.rs:44:5
[00:19:46]    |
[00:19:46] 44 |     pub unsafe fn new(inner: NonNull<dyn UnsafeWake>) -> Self {
[00:19:46]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:46]    |
[00:19:46]    = note: method `clone_raw`'s receiver cannot be dispatched on
[00:19:46]    = note: method `drop_raw`'s receiver cannot be dispatched on
[00:19:46]    = note: method `wake`'s receiver cannot be dispatched on
[00:19:46]    = note: method `wake_local`'s receiver cannot be dispatched on
[00:19:46] 
[00:19:46] error[E0038]: the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:46]    --> libcore/task/wake.rs:135:5
[00:19:46]     |
[00:19:46] 135 |     pub unsafe fn new(inner: NonNull<dyn UnsafeWake>) -> Self {
[00:19:46]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `task::wake::UnsafeWake` cannot be made into an object
[00:19:46]     |
[00:19:46]     = note: method `clone_raw`'s receiver cannot be dispatched on
[00:19:46]     = note: method `drop_raw`'s receiver cannot be dispatched on
[00:19:46]     = note: method `wake`'s receiver cannot be dispatched on
[00:19:46]     = note: method `wake_local`'s receiver cannot be dispatched on
travis_time:end:20ab82f4:start=1542209369251999781,finish=1542210557436047178,duration=1188184047397
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b3e077d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
