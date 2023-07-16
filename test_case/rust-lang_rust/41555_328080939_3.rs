
C:/Users/User/.cargo/bin/cargo.exe +nightly run --release
   Compiling safemem v0.2.0
   Compiling cfg-if v0.1.2
   Compiling matches v0.1.6
   Compiling libc v0.2.30
error[E0583]: file not found for module `macros`
  --> Z:\Users\User\.cargo\registry\src\github.com-1ecc6299db9ec823\libc-0.2.30\src\lib.rs:97:18
   |
97 | #[macro_use] mod macros;
   |                  ^^^^^^
   |
   = help: name the file either macros.rs or macros\mod.rs inside the directory "Z:\\Users\\User\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\libc-0.2.30\\src"

error: aborting due to previous error

   Compiling language-tags v0.2.2
error: Could not compile `libc`.
warning: build failed, waiting for other jobs to finish...
error: build failed

Process finished with exit code 101
