rust
// test `stat`
let err = fs::metadata("foo.txt").unwrap_err();
assert_eq!(err.kind(), ErrorKind::PermissionDenied); 
// check that it is the right kind of `PermissionDenied`
assert_eq!(err.raw_os_error(), Some(libc::EACCES)); 
