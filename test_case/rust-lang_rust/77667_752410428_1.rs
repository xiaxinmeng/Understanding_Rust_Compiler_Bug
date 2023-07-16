
   [...]
   Compiling nom v4.2.3
   Compiling buf-min v0.1.1
   Compiling combine v4.2.1
   Compiling quote v1.0.7
   Compiling v_escape_derive v0.8.1
   Compiling serde_yaml v0.8.13
   Compiling cheatsheet-gen v0.1.0 (D:\Development\Source\_thirdparty\rust-cheatsheet)
error: Stream Error: The stream is too short to perform the requested operation.
error: could not compile `cheatsheet-gen`

Caused by:
  process didn't exit successfully: `rustc --crate-name cheatsheet_gen --edition=2018 src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=db2d8889f40e140f --out-dir D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps -C linker=C:/Users/rb/scoop/shims/lld-link.exe -C incremental=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\incremental -L dependency=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps --extern bitflags=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libbitflags-808ca99e95faa8ae.rlib --extern combine=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libcombine-2ff8681a4a840a07.rlib --extern either_n=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libeither_n-5cbdae01f5acc47f.rlib --extern lazy_static=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\liblazy_static-a5fdf74fcdafbee9.rlib --extern serde=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libserde-e4c3e65b940c2046.rlib --extern serde_yaml=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libserde_yaml-38d2deeae8d2037a.rlib --extern v_htmlescape=D:\Development\Source\_thirdparty\rust-cheatsheet\target\debug\deps\libv_htmlescape-3ce086faae9d8288.rlib` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
