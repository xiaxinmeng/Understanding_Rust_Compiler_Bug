
error: Stream Error: The stream is too short to perform the requested operation.
error: could not compile `embedded-gui`

Caused by:
  process didn't exit successfully: `rustc --crate-name calculator --edition=2018 examples\calculator.rs 
--error-format=json 
--json=diagnostic-rendered-ansi 
--crate-type bin --emit=dep-info,link 
-C embed-bitcode=no -C debuginfo=2 
-C metadata=80352bbf14fdfebb 
--out-dir C:\_Hobby\RustLibraries\embedded-gui\target\debug\examples 
-C incremental=C:\_Hobby\RustLibraries\embedded-gui\target\debug\incremental 
-L dependency=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps 
--extern backend_embedded_graphics=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libbackend_embedded_graphics-8b6cd8529e0fc58d.rlib 
--extern embedded_graphics=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libembedded_graphics-d313357ca9d438a0.rlib 
--extern embedded_graphics_simulator=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libembedded_graphics_simulator-7dd88e0af16aed95.rlib 
--extern embedded_gui=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libembedded_gui-f62f849b7c07d6c3.rlib 
--extern heapless=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libheapless-50999711d995f99b.rlib 
--extern object_chain=C:\_Hobby\RustLibraries\embedded-gui\target\debug\deps\libobject_chain-96ea005c3c1e7f60.rlib`
 (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
