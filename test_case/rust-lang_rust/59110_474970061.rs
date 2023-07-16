
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl / x86_64-unknown-linux-musl)
          c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-musl" }, target: "x86_64-unknown-linux-musl" }
          > Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-musl" }, target: "x86_64-unknown-linux-gnu" }
            c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-musl" } }
          < Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-musl" }, target: "x86_64-unknown-linux-gnu" }
          > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-musl" } }
            c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
            > Rustc { target: "x86_64-unknown-linux-musl", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
              > Test { target: "x86_64-unknown-linux-musl", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                > Std { target: "x86_64-unknown-linux-musl", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-musl" }
                  < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-musl" }
                  > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-musl" }
                    c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
                  < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-musl" }
                  c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
                  c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
