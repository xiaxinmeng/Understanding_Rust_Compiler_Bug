
error: const items should never be `#[no_mangle]`
  --> $DIR/issue-45562.rs:11:14
   |
LL | #[no_mangle] pub const RAH: usize = 5;
   |              ---------^^^^^^^^^^^^^^^^ const items shouldn't be `#[no_mangle]`
   |              |
   |              help: try a static value: `pub static`
   |
   = note: #[deny(no_mangle_const_items)] on by default
