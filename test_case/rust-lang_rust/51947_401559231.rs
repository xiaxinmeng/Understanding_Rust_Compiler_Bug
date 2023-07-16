
$ cargo-bisect-rustc --start 56714acc5eb0687ed9a7566fdebe5528657fc5b3 --target thumbv7m-none-eabi --prompt --test-dir case/src/libcompiler_builtins --by-commit  -- build --target thumbv7m-none-eabi --release

searched toolchains 56714acc5eb0687ed9a7566fdebe5528657fc5b3 through 96b47337d9deebdb9cbca909e7772672062021e5
regression in e1151c9819cca90e511f60757297629177272d16
