
$ rustc r11469.rs 
r11469.rs:5:9: 5:14 error: type `Box<Foo>` cannot be dereferenced [E0033]
r11469.rs:5     let box x = box 1i as Box<Foo>;
                    ^~~~~
r11469.rs:6:9: 6:11 error: type `&Foo` cannot be dereferenced [E0033]
r11469.rs:6     let &x = &1i as &Foo;
                    ^~
error: aborting due to 2 previous errors
