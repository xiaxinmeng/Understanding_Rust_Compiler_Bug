

---- path::Path::file_name_1 stdout ----
    <anon>:4:21: 4:31 error: failed to resolve. Use of undeclared type or module `OsStr` [E0433]
<anon>:4     assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.").file_name());
                             ^~~~~~~~~~
<anon>:4:5: 4:81 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:4:45: 4:54 error: failed to resolve. Use of undeclared type or module `Path` [E0433]
<anon>:4     assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.").file_name());
                                                     ^~~~~~~~~
<anon>:4:5: 4:81 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:5:17: 5:27 error: failed to resolve. Use of undeclared type or module `OsStr` [E0433]
<anon>:5 assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.//").file_name());
                         ^~~~~~~~~~
<anon>:5:1: 5:79 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:5:41: 5:50 error: failed to resolve. Use of undeclared type or module `Path` [E0433]
<anon>:5 assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.//").file_name());
                                                 ^~~~~~~~~
<anon>:5:1: 5:79 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:6:18: 6:27 error: failed to resolve. Use of undeclared type or module `Path` [E0433]
<anon>:6 assert_eq!(None, Path::new("foo.txt/..").file_name());
                          ^~~~~~~~~
<anon>:6:1: 6:55 note: in this expansion of assert_eq! (defined in <std macros>)
error: aborting due to previous error(s)
thread 'path::Path::file_name_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:170
note: Run with `RUST_BACKTRACE=1` for a backtrace.


