plain
    Checking miniz_oxide v0.4.0
    Checking object v0.22.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
error[E0520]: `Item` specializes an item from a parent `impl`, but that item is not marked `default`
     |
     |
2452 | / impl<R: Read> Iterator for Bytes<R> {
2453 | |     type Item = Result<u8>;
2454 | |
2455 | |     fn next(&mut self) -> Option<Result<u8>> {
2465 | |     }
2466 | | }
2466 | | }
     | |_- parent `impl` is here
...
2469 |       type Item = Result<u8>;
     |       ^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `Item`
     |
     = note: to specialize, `Item` in the parent `impl` must be marked `default`

error[E0520]: `size_hint` specializes an item from a parent `impl`, but that item is not marked `default`
     |
     |
2452 | / impl<R: Read> Iterator for Bytes<R> {
2453 | |     type Item = Result<u8>;
2454 | |
2455 | |     fn next(&mut self) -> Option<Result<u8>> {
2465 | |     }
2466 | | }
2466 | | }
     | |_- parent `impl` is here
...
2471 | /     fn size_hint(&self) -> (usize, Option<usize>) {
2472 | |         match self.inner.metadata() {
2473 | |             Ok(metadata) => {
2474 | |                 let file_length = metadata.len() as usize;
2478 | |         }
2479 | |     }
2479 | |     }
     | |_____^ cannot specialize default item `size_hint`
     |
     = note: to specialize, `size_hint` in the parent `impl` must be marked `default`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0520`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:45
