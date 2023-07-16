
% cargo +nightly-2023-01-28 doc   
   Compiling flash-algorithm v0.3.0
 Documenting flash-algorithm v0.3.0
 Documenting testme v0.1.0 (/Users/tmgross/Documents/Projects/testme)
error[E0152]: found duplicate lang item `panic_impl`
 --> src/main.rs:5:1
  |
5 | fn panic(_: &core::panic::PanicInfo) -> ! {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: the lang item is first defined in crate `flash_algorithm`.
  = note: first definition in `flash_algorithm` loaded from /Users/tmgross/Documents/Projects/testme/target/debug/deps/libflash_algorithm-53fc8a5d1bd6046f.rmeta
  = note: second definition in the local crate (`testme`)

For more information about this error, try `rustc --explain E0152`.
error: could not document `testme`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type bin --crate-name testme src/main.rs -o /Users/tmgross/Documents/Projects/testme/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=181 --document-private-items '-Arustdoc::private-intra-doc-links' -C metadata=179b40203cf7ab78 -L dependency=/Users/tmgross/Documents/Projects/testme/target/debug/deps --extern flash_algorithm=/Users/tmgross/Documents/Projects/testme/target/debug/deps/libflash_algorithm-53fc8a5d1bd6046f.rmeta --crate-version 0.1.0` (exit status: 1)

% cargo doc
 Documenting flash-algorithm v0.3.0
 Documenting testme v0.1.0 (/Users/tmgross/Documents/Projects/testme)
    Finished dev [unoptimized + debuginfo] target(s) in 1.91s
