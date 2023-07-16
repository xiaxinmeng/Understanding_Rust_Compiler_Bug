
$ ./x.py build --dry-run -v
[...]
running: /home/bjorn/Documenten/rust/build/bootstrap/debug/bootstrap build --dry-run -v
thread 'main' panicked at 'std::fs::read_to_string(ci_llvm.join("link-type.txt")) failed with No such file or directory (os error 2)', src/bootstrap/config.rs:823:33
stack backtrace:
   0: rust_begin_unwind
             at /rustc/05b6023675d77979637b04a350c85903fbf59257/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/05b6023675d77979637b04a350c85903fbf59257/library/std/src/panicking.rs:435:5
   2: bootstrap::config::Config::parse
             at ./src/bootstrap/config.rs:823:33
   3: bootstrap::main
             at ./src/bootstrap/bin/main.rs:14:18
