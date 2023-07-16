$ ./x.py fmt
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized] target(s) in 0.08s
error: expected expression, found reserved keyword `try`
  --> /home/shahn/coding/git/rust/cargo/src/bin/cargo.rs:85:28
   |
85 |         let args: Vec<_> = try!(env::args_os()
   |                            ^^^ expected expression

Running `"/home/shahn/coding/git/rust/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/home/shahn/coding/git/rust" "--edition" "2018" "--unstable-features" "--skip-children" "/home/shahn/coding/git/rust/cargo/src/bin/cargo.rs"` failed.
If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
failed to run: /home/shahn/coding/git/rust/build/bootstrap/debug/bootstrap fmt
Build completed unsuccessfully in 0:00:03
