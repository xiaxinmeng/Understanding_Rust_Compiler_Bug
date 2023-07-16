
    assert_that(bar.cargo("build").arg("--verbose"),
                execs().with_stderr_contains("\
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] -C ar=nonexistent-ar -C linker=nonexistent-linker [..]`
[ERROR] could not exec the linker [..]
"));
