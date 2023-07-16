
$ rustc -v
rustc 0.12.0-pre-nightly (cf1381c1d 2014-07-26 00:46:16 +0000)

$ rustc c.rs 
m1.rs:13:9: 13:20 error: cannot borrow data mutably in a `&` reference 
m1.rs:13         self.reader.read(vec);
                 ^~~~~~~~~~~ 
m1.rs:19:26: 19:27 error: `r` does not live long enough
m1.rs:19     Container::wrap(&mut r as &mut Reader)
                                  ^
m1.rs:17:41: 20:2 note: reference must be valid for the lifetime 'a as defined on the block at 17:40...
m1.rs:17 pub fn for_stdin<'a>() -> Container<'a> { 
m1.rs:18     let mut r = io::stdin();
m1.rs:19     Container::wrap(&mut r as &mut Reader)
m1.rs:20 }
m1.rs:17:41: 20:2 note: ...but borrowed value is only valid for the block at 17:40
m1.rs:17 pub fn for_stdin<'a>() -> Container<'a> { 
m1.rs:18     let mut r = io::stdin();
m1.rs:19     Container::wrap(&mut r as &mut Reader)
m1.rs:20 }
error: aborting due to 2 previous errors
