
bar.rs:6:20: 6:27 error: use of moved value: `f.field` [E0382]
bar.rs:6     println!("{}", f.field);
                            ^~~~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
bar.rs:6:5: 6:29 note: in this expansion of println! (defined in <std macros>)
bar.rs:6:20: 6:27 help: run `rustc --explain E0382` to see a detailed explanation
bar.rs:5:10: 5:11 note: `f` moved here because it has type `foo::inner::Foo`, which is non-copyable
bar.rs:5     drop(f);
                  ^
error: aborting due to previous error
