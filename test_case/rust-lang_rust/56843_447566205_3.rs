\n\nIf we control the definition of a type, we can implement `Clone` on it ourselves\nwith `#[derive(Clone)]`.\n\nSome types have no ownership semantics at all and are trivial to duplicate. An\nexample is `i32` and the other number types. We don't have to call `.clone()` to\nclone them, because they are marked `Copy` in addition to `Clone`.  Implicit\ncloning is more convenient in thistravis_time:end:2fff3253:start=1544875369708468589,finish=1544878167048561932,duration=2797340093343
travis_time:start:01cc033e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 12:49:27 UTC 2018
Sat, 15 Dec 2018 12:49:27 GMT
