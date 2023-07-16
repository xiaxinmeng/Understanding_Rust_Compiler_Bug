
info: generating a diff against nightly rustdoc

error: failed to run nightly rustdoc
status: exit code: 1
command: "rustdoc" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646/auxiliary" "-o" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646.nightly" "/home/joshua/rustc/src/test/rustdoc/issue-20646.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0514]: found crate `issue_20646` compiled by an incompatible version of rustc
 --> /home/joshua/rustc/src/test/rustdoc/issue-20646.rs:6:1
  |
6 | extern crate issue_20646;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: please recompile that crate using this compiler (rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24))
  = note: the following crate versions were found:
          crate `issue_20646` compiled by rustc 1.49.0-dev: /home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20646/auxiliary/libissue_20646.so

error: aborting due to previous error
