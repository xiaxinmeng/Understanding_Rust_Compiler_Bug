
❯ ./x test ui --stage 0 --bless
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
error: `--stage 0` runs compiletest on the beta compiler, not your local changes, and will almost always cause tests to fail
help: to test the compiler, use `--stage 1` instead
help: to test the standard library, use `--stage 0 library/std` instead
note: if you're sure you want to do this, please open an issue as to why. In the meantime, you can override this with `COMPILETEST_FORCE_STAGE0=1`.
