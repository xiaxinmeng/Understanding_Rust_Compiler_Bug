 sh
$ bash -c 'rustc lib.rs && rustc main.rs -L.'
…
note: main.o: In function `error::Error::produce::h13795623746139935984':
main.0.rs:(.text._ZN5error5Error7produce21h13795623746139935984E+0x3d): undefined reference to `error::Error::produce::LINK_ERROR::h6790b04a8ae1707fTaa'
collect2: error: ld returned 1 exit status
