
$ rustc --test ./hygienic_macro_test.rs
./hygienic_macro_test.rs:8:7: 8:8 warning: value assigned to `x` is never read
./hygienic_macro_test.rs:8       $i = $i - x;
                                  ^
./hygienic_macro_test.rs:3:0: 11:1 note: in expansion of #refer_to_x
./hygienic_macro_test.rs:17:2: 17:17 note: expansion site
