
$ echo '#![crate_name = "foo"] fn main() {}' | rustc - --crate-name bar
error: `--crate-name` and `#[crate_name]` are required to match, but `bar` != `foo`
 --> <anon>:1:1
  |
1 | #![crate_name = "foo"] fn main() {}
  | ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
