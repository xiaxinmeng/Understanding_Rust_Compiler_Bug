
% cat -n /tmp/foo.rs
     1  #[link(name="foo", vers="0.0")];
     2  
     3  #[inline(always)]
     4  pub fn access_inline_always(a: &[int], i: uint) -> int {
     5      info!("foo::access_inline_always");
     6      a[i]
     7  }
     8  
     9  #[inline(never)]
    10  pub fn access_inline_never(a: &[int], i: uint) -> int {
    11      info!("foo::access_inline_never");
    12      a[i]
    13  }
% cat -n /tmp/bar.rs
     1  extern mod foo;
     2  
     3  fn main() {
     4      let arr = [1, 2, 3];
     5      let a_0 = foo::access_inline_never(arr, 0);
     6      let a_1 = foo::access_inline_always(arr, 1);
     7      println(fmt!("arr[0]: %? arr[1]: %?", a_0, a_1));
     8      let a_4 = foo::access_inline_always(arr, 4);
     9      println(fmt!("arr[4]: %?", a_4));
    10  }
% rustc --cfg debug --lib /tmp/foo.rs && rustc -L/tmp /tmp/bar.rs && RUST_LOG=foo=4 /tmp/bar
warning: no debug symbols in executable (-arch x86_64)
warning: no debug symbols in executable (-arch x86_64)
rust: ~"\"foo::access_inline_never\""
arr[0]: 1 arr[1]: 2
rust: task failed at 'index out of bounds: the len is 3 but the index is 4', /tmp/bar.rs:1
rust: domain main @0x7ff333807810 root task failed
% rustc --cfg debug --lib /tmp/foo.rs && rustc -L/tmp /tmp/bar.rs && RUST_LOG=bar=4 /tmp/bar
warning: no debug symbols in executable (-arch x86_64)
warning: no debug symbols in executable (-arch x86_64)
rust: ~"\"foo::access_inline_always\""
arr[0]: 1 arr[1]: 2
rust: ~"\"foo::access_inline_always\""
rust: task failed at 'index out of bounds: the len is 3 but the index is 4', /tmp/bar.rs:1
rust: domain main @0x7fc1b8807810 root task failed
% 
