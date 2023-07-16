
$ rustc test.rs
test.rs:17:17: 17:38 error: expected constant integer for repeat count, found no
n-constant expression [E0307]
test.rs:17     let c = [0; NESTED_S.nested.field];
                           ^~~~~~~~~~~~~~~~~~~~~
test.rs:18:17: 18:31 error: expected constant integer for repeat count, found no
n-constant expression [E0307]
test.rs:18     let d = [0; (NESTED_T.0).0];
                           ^~~~~~~~~~~~~~
test.rs:19:17: 19:41 error: expected constant integer for repeat count, found no
n-constant expression [E0307]
test.rs:19     let e = [0; (MIX_1.0).0.nested.field];
                           ^~~~~~~~~~~~~~~~~~~~~~~~
test.rs:20:17: 20:35 error: expected constant integer for repeat count, found no
n-constant expression [E0307]
test.rs:20     let f = [0; (MIX_2.nested.0).0];
                           ^~~~~~~~~~~~~~~~~~
test.rs:21:17: 21:26 error: expected constant integer for repeat count, found va
riable [E0307]
test.rs:21     let g = [0; INSTANT_1];
                           ^~~~~~~~~
test.rs:22:17: 22:26 error: expected constant integer for repeat count, found va
riable [E0307]
test.rs:22     let h = [0; INSTANT_2];
                           ^~~~~~~~~
error: aborting due to 6 previous errors
