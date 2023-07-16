
The actual stderr differed from the expected stderr.
Actual stderr saved to /home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full/const-argument-non-static-lifetime.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-argument-non-static-lifetime.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit status: 0
command: "/home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/home/jnelson/rust-lang/rust/src/test/ui/const-generics/const-argument-non-static-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full/a" "--out-dir" "/home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/home/jnelson/rust-lang/rust/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full/auxiliary"
stdout: none
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
------------------------------------------



failures:
    [ui] src/test/ui/const-generics/const-argument-non-static-lifetime.rs#full
