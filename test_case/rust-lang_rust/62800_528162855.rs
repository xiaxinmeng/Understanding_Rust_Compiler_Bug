plain
2019-09-05T01:56:52.8631531Z test vec.rs - vec::Vec<T>::with_capacity (line 339) ... ok
2019-09-05T01:56:52.8634946Z 
2019-09-05T01:56:52.8638137Z failures:
2019-09-05T01:56:52.8639846Z 
2019-09-05T01:56:52.8649490Z ---- prelude/mod.rs - prelude (line 6) stdout ----
2019-09-05T01:56:52.8651457Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-09-05T01:56:52.8651840Z  --> prelude/mod.rs:8:1
2019-09-05T01:56:52.8652019Z 5 | extern crate alloc;
2019-09-05T01:56:52.8652507Z   | ^^^^^^^^^^^^^^^^^^^
2019-09-05T01:56:52.8652589Z 
2019-09-05T01:56:52.8652656Z error: aborting due to previous error
2019-09-05T01:56:52.8652656Z error: aborting due to previous error
2019-09-05T01:56:52.8652720Z 
2019-09-05T01:56:52.8653060Z Couldn't compile the test.
2019-09-05T01:56:52.8653403Z ---- raw_vec.rs - raw_vec::RawVec<T, A>::double (line 261) stdout ----
2019-09-05T01:56:52.8654097Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-09-05T01:56:52.8654844Z  --> raw_vec.rs:263:1
2019-09-05T01:56:52.8655005Z 4 | extern crate alloc;
2019-09-05T01:56:52.8655214Z   | ^^^^^^^^^^^^^^^^^^^
2019-09-05T01:56:52.8655270Z 
2019-09-05T01:56:52.8655355Z error: aborting due to previous error
2019-09-05T01:56:52.8655355Z error: aborting due to previous error
2019-09-05T01:56:52.8655402Z 
2019-09-05T01:56:52.8655705Z Couldn't compile the test.
2019-09-05T01:56:52.8656017Z ---- raw_vec.rs - raw_vec::RawVec<T, A>::reserve (line 466) stdout ----
2019-09-05T01:56:52.8656482Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-09-05T01:56:52.8656775Z  --> raw_vec.rs:468:1
2019-09-05T01:56:52.8656934Z 4 | extern crate alloc;
2019-09-05T01:56:52.8657021Z   | ^^^^^^^^^^^^^^^^^^^
2019-09-05T01:56:52.8657078Z 
2019-09-05T01:56:52.8657161Z error: aborting due to previous error
---
2019-09-05T01:56:52.8811600Z == clock drift check ==
2019-09-05T01:56:52.8828346Z   local time: Thu Sep  5 01:56:52 UTC 2019
2019-09-05T01:56:53.0500684Z   network time: Thu, 05 Sep 2019 01:56:53 GMT
2019-09-05T01:56:53.0503670Z == end clock drift check ==
2019-09-05T01:56:53.5693162Z ##[error]Bash exited with code '1'.
2019-09-05T01:56:53.5770296Z ##[section]Starting: Upload CPU usage statistics
2019-09-05T01:56:53.5775860Z ==============================================================================
2019-09-05T01:56:53.5775991Z Task         : Bash
2019-09-05T01:56:53.5776060Z Description  : Run a Bash script on macOS, Linux, or Windows
