
$ rustc --explain E0106
!! executing '/home/zazdxscf/build/1nonpkgs/rust/rust//x86_64-unknown-linux-gnu/stage1/bin//rustc' with args: '--explain E0106'
This error indicates that a lifetime is missing from a type. If it is an error
inside a function signature, the problem may be with failing to adhere to the
lifetime elision rules (see below).

Here are some simple examples of where you'll run into this error:
