 shell
$ rustc main.rs
main.rs:8:33: 8:44 error: internal compiler error: constant expression should not reach expr::trans_def
main.rs:8     println!("offset of c {}" , OFFSET_OF_C as usize)
