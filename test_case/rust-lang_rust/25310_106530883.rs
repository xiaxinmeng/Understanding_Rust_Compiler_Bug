
<anon>:12:22: 12:25 error: type mismatch: the type `fn(_) -> core::option::Option<std::ffi::os_str::OsString> {std::env::var_os}` implements the trait `core::ops::Fn<(_,)>`, but the trait `for<'r> core::ops::Fn<(&'r str,)>` is required (expected concrete lifetime, found bound lifetime parameter ) [E0281]
<anon>:12     println!("{:?}", foo(&env::var_os));
                               ^~~
