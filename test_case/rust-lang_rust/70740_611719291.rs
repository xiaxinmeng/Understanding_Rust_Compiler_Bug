console
❯ rustc --target x86_64-unknown-linux-musl -C relocation-model=pic test-aslr.rs

❯ file test-aslr
test-aslr: ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=02269beb7670fe78358c44c07d25bf524a48fbda, with debug_info, not stripped, too many notes (256)

❯ ./test-aslr --test-aslr
PASS: ./test-aslr does ASLR

❯ rustc --target x86_64-unknown-linux-musl -C relocation-model=static test-aslr.rs

❯ file test-aslr
test-aslr: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=9158a66db26706fc75e6b1e46c5d60b5f25b9ca2, with debug_info, not stripped, too many notes (256)

❯ ./test-aslr --test-aslr
FAIL: ./test-aslr most likely no ASLR

❯ ./test-aslr --test-no-aslr
PASS: ./test-aslr does no ASLR
