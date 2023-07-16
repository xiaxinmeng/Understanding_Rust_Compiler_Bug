
[01:14:26] error[E0379]: trait fns cannot be declared const
[01:14:26]    |
[01:14:26]    |
[01:14:26] LL |     const fn const_val<T: Sized>() -> usize {
[01:14:26]    |     ^^^^^ trait fns cannot be const
[01:14:26] error[E0019]: constant contains unimplemented expression type
[01:14:26]   --> /checkout/src/test/ui/issues/issue-54954.rs:3:24
[01:14:26]    |
[01:14:26]    |
[01:14:26] LL | const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
[01:14:26] 
[01:14:26] thread 'rustc' panicked at 'attempt to shift left with overflow', src/librustc/mir/interpret/mod.rs:436:5
