bash
$ cargo build
   Compiling bisect v0.1.0 (/home/habbasi/bisect)
error: expected one of `,` or `>`, found keyword `as`
 --> src/lib.rs:6:45
  |
6 |     pub vulkan_device : ash::version::<Type as InstanceV1_0>::Device
  |                                             ^^ expected one of `,` or `>`
  |
help: expressions must be enclosed in braces to be used as const generic arguments
  |
6 |     pub vulkan_device : ash::version::<{ Type as InstanceV1_0 }>::Device
  |                                        ^                      ^

error[E0433]: failed to resolve: use of undeclared crate or module `vk`
 --> src/lib.rs:5:27
  |
5 |     pub physical_device : vk::PhysicalDevice,
  |                           ^^ use of undeclared crate or module `vk`

error[E0412]: cannot find type `Device` in module `ash::version`
 --> src/lib.rs:6:63
  |
6 |     pub vulkan_device : ash::version::<Type as InstanceV1_0>::Device
  |                                                               ^^^^^^ not found in `ash::version`
  |
help: consider importing one of these items
  |
1 | use ash::Device;
  |
1 | use ash::vk::Device;
  |

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/decoder.rs:900:54
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0 (2fd73fabe 2021-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [generics_of] computing generics of `ash::version`
#1 [opt_const_param_of] computing the optional const parameter of `GfxDevice::vulkan_device::{constant#0}`
end of query stack
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `bisect`

To learn more, run the command again with --verbose.
