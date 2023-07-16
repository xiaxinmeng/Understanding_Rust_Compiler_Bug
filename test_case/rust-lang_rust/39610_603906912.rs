
error: failed to run custom build command for `libc v0.2.67`

Caused by:
  process didn't exit successfully: `/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build` (exit code: 77)
--- stderr
==1776==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x561b64de1400  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x69400)
    #1 0x561b64ddf1a7  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x671a7)
    #2 0x561b64e06865  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x8e865)
    #3 0x561b64e19ece  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0xa1ece)
    #4 0x561b64e067ab  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x8e7ab)
    #5 0x561b64de4001  (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x6c001)


SUMMARY: MemorySanitizer: use-of-uninitialized-value (/tmp/test-crate/target/debug/build/libc-eddab43bf2017670/build-script-build+0x69400)
Exiting
