
lunch-box. rustc --stage0 ~/tmp/issue-21664.rs
/home/nmatsakis/tmp/issue-21664.rs:20:5: 20:13 error: type mismatch resolving `<T as FnLike<u32>>::R == bool`:
 expected associated type,
    found bool
/home/nmatsakis/tmp/issue-21664.rs:20     bar::<T>();
                                          ^~~~~~~~
/home/nmatsakis/tmp/issue-21664.rs:20:5: 20:13 note: required by `bar`
/home/nmatsakis/tmp/issue-21664.rs:20     bar::<T>();
                                          ^~~~~~~~
error: aborting due to previous error
