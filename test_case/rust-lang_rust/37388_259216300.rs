
lunch-box. cat > ~/tmp/issue-37388.rs
fn foo(x: &i32) {
    bar(x); // don't want a hint here!
}
fn bar(x: &mut i32) {}
fn main() { }

lunch-box. rustc --stage1 ~/tmp/issue-37388.rs
error[E0308]: mismatched types
 --> /home/nmatsakis/tmp/issue-37388.rs:2:9
  |
2 |     bar(x); // don't want a hint here!
  |         ^ types differ in mutability
  |
  = note: expected type `&mut i32`
  = note:    found type `&i32`
  = help: try with `&mut x`

error: aborting due to previous error
