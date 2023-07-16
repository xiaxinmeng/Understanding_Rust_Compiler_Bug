
/home/nmatsakis/tmp/foo.rs:13:42: 13:44 error: `rc` does not live long enough
/home/nmatsakis/tmp/foo.rs:13             CachedMir::Owned(ref rc) => &rc,
                                                                       ^~
/home/nmatsakis/tmp/foo.rs:10:44: 15:6 note: reference must be valid for the lifetime 'a as defined on the block at 10:43...
/home/nmatsakis/tmp/foo.rs:10     fn get_ref<'a>(&'a self) -> &'a String {
/home/nmatsakis/tmp/foo.rs:11         match *self {
/home/nmatsakis/tmp/foo.rs:12             CachedMir::Ref(r) => r,
/home/nmatsakis/tmp/foo.rs:13             CachedMir::Owned(ref rc) => &rc,
/home/nmatsakis/tmp/foo.rs:14         }
/home/nmatsakis/tmp/foo.rs:15     }
/home/nmatsakis/tmp/foo.rs:11:9: 14:10 note: ...but borrowed value is only valid for the match at 11:8
/home/nmatsakis/tmp/foo.rs:11         match *self {
/home/nmatsakis/tmp/foo.rs:12             CachedMir::Ref(r) => r,
/home/nmatsakis/tmp/foo.rs:13             CachedMir::Owned(ref rc) => &rc,
/home/nmatsakis/tmp/foo.rs:14         }
error: aborting due to previous error
