
$ RUSTFLAGS="-Zremap-path-prefix-from=/Users -Zremap-path-prefix-to=/Hello" cargo +nightly build
   Compiling dtoa v0.4.2
   Compiling adler32 v1.0.2
   Compiling scoped-tls v0.1.0
   Compiling semver v0.1.20
   Compiling matches v0.1.6
   Compiling futures v0.1.15
   Compiling httparse v1.2.3
   Compiling num-traits v0.1.40
error[E0583]: file not found for module `diyfp`
  --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/dtoa-0.4.2/src/lib.rs:11:18
   |
11 | #[macro_use] mod diyfp;
   |                  ^^^^^
error[E0583]: file not found for module `version`
   --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/semver-0.1.20/src/lib.rs:173:5
    |
173 | mod version;
    |     ^^^^^^^
   |
    |
   = help: name the file either diyfp.rs or diyfp/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/dtoa-0.4.2/src"
    = help: name the file either version.rs or version/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/semver-0.1.20/src"


error[E0583]: file not found for module `identities`
  --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.1.40/src/lib.rs:32:9
   |
32 | pub mod identities;
   |         ^^^^^^^^^^
error: aborting due to previous error
   |

error: aborting due to previous error

   = help: name the file either identities.rs or identities/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.1.40/src"

error: aborting due to previous error

error[E0583]: file not found for module `poll`
   --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.15/src/lib.rs:173:5
    |
173 | mod poll;
    |     ^^^^
    |
    = help: name the file either poll.rs or poll/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.15/src"

error: aborting due to previous error

error: Could not compile `dtoa`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `semver`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `num-traits`.
warning: build failed, waiting for other jobs to finish...
error: Could not compile `futures`.
warning: build failed, waiting for other jobs to finish...
error[E0583]: file not found for module `iter`
  --> /Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/httparse-1.2.3/src/lib.rs:20:5
   |
20 | mod iter;
   |     ^^^^
   |
   = help: name the file either iter.rs or iter/mod.rs inside the directory "/Hello/hagen/.cargo/registry/src/github.com-1ecc6299db9ec823/httparse-1.2.3/src"

error: aborting due to previous error

error: Could not compile `httparse`.
warning: build failed, waiting for other jobs to finish...
error: build failed
