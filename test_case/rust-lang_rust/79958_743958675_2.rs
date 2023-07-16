text
===================================================================================================

Failure due to:
    LLVM Profile Error: Failed to write file "default.profraw": Permission denied

The test must be compiling in some kind of sandbox. I was able to work around this problem by
prefixing the `./x.py test` command with `LLVM_PROFILE_FILE=/tmp/command-pre-exec2.profraw`

src/test/ui/command/command-pre-exec.rs

