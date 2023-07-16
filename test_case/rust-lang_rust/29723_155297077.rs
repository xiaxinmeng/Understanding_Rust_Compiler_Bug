 rust
match foo_opt {
  Some(ref foo) if cond => { ... }
  None => { foo_opt.val = bar; }
}
