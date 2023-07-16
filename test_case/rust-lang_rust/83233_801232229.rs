plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0425]: cannot find value `M` in this scope
     |
     |
1671 |     pub fn split_array<const N: usize>(&self) -> (&[T; N], &[T]) {
     |                              - similarly named const parameter `N` defined here
1672 |         let (a, b) = self.split_at(M);
     |                                    ^ help: a const parameter with a similar name exists: `N`

error[E0425]: cannot find value `M` in this scope
     |
     |
1699 |     pub fn split_array_mut<const N: usize>(&mut self) -> (&mut [T; N], &mut [T]) {
     |                                  - similarly named const parameter `N` defined here
1700 |         let (a, b) = self.split_at_mut(M);
     |                                        ^ help: a const parameter with a similar name exists: `N`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:41
