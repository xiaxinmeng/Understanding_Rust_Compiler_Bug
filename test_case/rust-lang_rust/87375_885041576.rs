plain
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
error[E0277]: the trait bound `NonZeroU32: Ord` is not satisfied
   --> library/proc_macro/src/bridge/handle.rs:22:37
    |
22  |         OwnedStore { counter, data: BTreeMap::new() }
    |                                     ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `NonZeroU32`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <NonZeroU32 as Ord>
note: required by `BTreeMap::<K, V>::new`
    |
    |
490 | /     pub const fn new() -> BTreeMap<K, V>
491 | |     where
492 | |         K: Ord,


error[E0277]: the trait bound `NonZeroU32: Ord` is not satisfied
  --> library/proc_macro/src/bridge/handle.rs:30:34
   |
30 |         assert!(self.data.insert(handle, x).is_none());
   |                                  ^^^^^^ the trait `Ord` is not implemented for `NonZeroU32`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <NonZeroU32 as Ord>

error[E0277]: the trait bound `NonZeroU32: Ord` is not satisfied
  --> library/proc_macro/src/bridge/handle.rs:35:19
   |
35 |         self.data.remove(&h).expect("use-after-free in `proc_macro` handle")
   |                   ^^^^^^ the trait `Ord` is not implemented for `NonZeroU32`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <NonZeroU32 as Ord>

error[E0277]: the trait bound `NonZeroU32: Ord` is not satisfied
  --> library/proc_macro/src/bridge/handle.rs:42:19
   |
42 |         self.data.get(&h).expect("use-after-free in `proc_macro` handle")
   |                   ^^^ the trait `Ord` is not implemented for `NonZeroU32`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <NonZeroU32 as Ord>

error[E0277]: the trait bound `NonZeroU32: Ord` is not satisfied
  --> library/proc_macro/src/bridge/handle.rs:48:19
   |
48 |         self.data.get_mut(&h).expect("use-after-free in `proc_macro` handle")
   |                   ^^^^^^^ the trait `Ord` is not implemented for `NonZeroU32`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <NonZeroU32 as Ord>
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `proc_macro`
