shell
cohenarthur@gcc14:~/gccrs$ build/gcc/rust1 test.rs
rust1: fatal error: gccrs is not yet able to compile Rust code properly. Most of the errors produced will be gccrs' fault and not the crate you are trying to compile. Because of this, please reports issues to us directly instead of opening issues on said crate's repository.

Our github repository: https://github.com/rust-gcc/gccrs
Our bugzilla tracker: https://gcc.gnu.org/bugzilla/buglist.cgi?bug_status=__open__&component=rust&product=gcc

If you understand this, and understand that the binaries produced might not behave accordingly, you may attempt to use gccrs in an experimental manner by passing the following flag:

`-frust-incomplete-and-experimental-compiler-do-not-use`

For cargo-gccrs, this means passing

GCCRS_EXTRA_FLAGS="-frust-incomplete-and-experimental-compiler-do-not-use"

as an environment variable.
compilation terminated.
cohenarthur@gcc14:~/gccrs$ build/gcc/rust1 test.rs -frust-incomplete-and-experimental-compiler-do-not-use
