
rustcargo@zealot:~$ cat t.rs
pub fn thing<'a>(_s: &'a str) where 'a: 'static { }
fn main() { thing(""); }
rustcargo@zealot:~$ rustc +nightly t.rs
warning: unnecessary lifetime parameter `'a`
 --> t.rs:1:37
  |
1 | pub fn thing<'a>(_s: &'a str) where 'a: 'static { }
  |                                     ^^
  |
  = help: you can use the `'static` lifetime directly, in place of `'a`

warning: 1 warning emitted

rustcargo@zealot:~$ 
