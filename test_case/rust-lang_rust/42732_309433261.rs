
[00:14:23] error[E0433]: failed to resolve. Use of undeclared type or module `Stdio`
[00:14:23]    --> /checkout/src/librustc_driver/lib.rs:391:21
[00:14:23]     |
[00:14:23] 391 |             .stdout(Stdio::from_raw_fd(raw_file_descriptor_id))
[00:14:23]     |                     ^^^^^^^^^^^^^^^^^^ Use of undeclared type or module `Stdio`
[00:14:23] 
[00:14:23] error: aborting due to previous error
