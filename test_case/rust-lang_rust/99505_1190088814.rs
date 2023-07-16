plain
    Checking addr2line v0.16.0
error[E0425]: cannot find value `init_state` in this scope
   --> library/std/src/sys_common/once/futex.rs:115:57
    |
115 |                     waiter_queue.set_state_on_drop_to = init_state.set_state_to.get();

error: implementation has missing stability attribute
   --> library/std/src/sync/once.rs:319:1
    |
    |
319 | / impl fmt::Debug for OnceState {
320 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
321 | |         f.debug_struct("OnceState").field("poisoned", &self.is_poisoned()).finish()
323 | | }
    | |_^

error[E0624]: associated function `call` is private
error[E0624]: associated function `call` is private
   --> library/std/src/sync/once.rs:147:20
    |
147 |         self.inner.call(false, &mut |_| f.take().unwrap()());
    |
   ::: library/std/src/sys_common/once/futex.rs:89:5
    |
    |
89  |     fn call(&self, ignore_poisoning: bool, f: &mut impl FnMut(&OnceState)) {

error[E0624]: associated function `call` is private
   --> library/std/src/sync/once.rs:206:20
    |
    |
206 |         self.inner.call(true, &mut |p| f.take().unwrap()(OnceState::from_inner(p)));
    |
   ::: library/std/src/sys_common/once/futex.rs:89:5
    |
    |
89  |     fn call(&self, ignore_poisoning: bool, f: &mut impl FnMut(&OnceState)) {


error[E0599]: no method named `is_poisoned` found for struct `sys_common::once::futex::OnceState` in the current scope
    |
308 |         self.inner.is_poisoned()
    |                    ^^^^^^^^^^^ help: there is an associated function with a similar name: `poison`
    |
    |
   ::: library/std/src/sys_common/once/futex.rs:30:1
    |
30  | pub struct OnceState {
    | -------------------- method `is_poisoned` not found for this
error[E0308]: mismatched types
  --> library/std/src/sys_common/once/futex.rs:70:23
   |
70 |         Once { state: INCOMPLETE }
70 |         Once { state: INCOMPLETE }
   |                       ^^^^^^^^^^ expected struct `AtomicU32`, found `u32`

error[E0369]: no implementation for `core::result::Result<u32, _> | core::result::Result<_, u32>`
   --> library/std/src/sys_common/once/futex.rs:119:31
    |
119 |                     Ok(state) | Err(state) =
    |                     --------- ^ ---------- core::result::Result<_, u32>
    |                     |
    |                     core::result::Result<u32, _>

error[E0070]: invalid left-hand side of assignment
   --> library/std/src/sys_common/once/futex.rs:119:44
    |
119 |                     Ok(state) | Err(state) =
    |                     |
    |                     cannot assign to this expression

Some errors have detailed explanations: E0070, E0308, E0369, E0425, E0599, E0624.
