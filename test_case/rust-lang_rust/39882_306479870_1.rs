
$ RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-apple-darwin
...
=================================================================
==89645==ERROR: AddressSanitizer: 6?E on address 0x7fff5aac4508 at pc 0x0001051796bb bp 0x7fff5aac44d0 sp 0x7fff5aac44c8
ACCESS of size 0 at 0x7fff5aac4508 thread T0
...
