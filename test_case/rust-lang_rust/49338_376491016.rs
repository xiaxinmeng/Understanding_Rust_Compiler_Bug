
> python x.py --help --verbose
info: Downloading and building bootstrap before processing --help
      command. See src/bootstrap/README.md for help with common
      commands.
running: C:\Source\rust_test\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path C:\Source\rust_test\src/bootstrap/Cargo.toml
error: failed to read `C:\Source\rust_test\src\tools\clippy\Cargo.toml`

Caused by:
  The system cannot find the file specified. (os error 2)
Traceback (most recent call last):
  File "x.py", line 20, in <module>
    bootstrap.main()
  File "C:\Source\rust_test\src\bootstrap\bootstrap.py", line 800, in main
    bootstrap(help_triggered)
  File "C:\Source\rust_test\src\bootstrap\bootstrap.py", line 780, in bootstrap
    build.build_bootstrap()
  File "C:\Source\rust_test\src\bootstrap\bootstrap.py", line 606, in build_bootstrap
    run(args, env=env, verbose=self.verbose)
  File "C:\Source\rust_test\src\bootstrap\bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: C:\Source\rust_test\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build --manifest-path C:\Source\rust_test\src/bootstrap/Cargo.toml
