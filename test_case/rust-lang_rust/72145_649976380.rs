
    Updating crates.io index
  Downloaded sd v0.7.5
  Downloaded 1 crate (14.8 KB) in 0.93s
  Installing sd v0.7.5
  Downloaded aho-corasick v0.7.13
  [...]
   Compiling structopt v0.3.15
   Compiling sd v0.7.5
error: failed to compile `sd v0.7.5`, intermediate artifacts can be found at `C:\Users\Chris\AppData\Local\Temp\cargo-install8LONA5`

Caused by:
  could not compile `sd`.

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2018 C:\Users\Chris\.cargo\registry\src\github.com-1ecc6299db9ec823\sd-0.7.5\build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -Cembed-bitcode=no -C codegen-units=1 -C metadata=d011dbc6f5f1cce2 -C extra-filename=-d011dbc6f5f1cce2 --out-dir C:\Users\Chris\AppData\Local\Temp\cargo-install8LONA5\release\build\sd-d011dbc6f5f1cce2 -L dependency=C:\Users\Chris\AppData\Local\Temp\cargo-install8LONA5\release\deps --extern man=C:\Users\Chris\AppData\Local\Temp\cargo-install8LONA5\release\deps\libman-b99848b80d918758.rlib --extern structopt=C:\Users\Chris\AppData\Local\Temp\cargo-install8LONA5\release\deps\libstructopt-ee882b02652d4990.rlib --cap-lints allow -Clinker-flavor=lld-link -Ctarget-feature=+avx` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
