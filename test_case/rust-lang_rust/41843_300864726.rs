
  = note: LINK : warning LNK4098: defaultlib 'MSVCRT' conflicts with use of other libs; use /NODEFAULTLIB:library
          liblzma_sys-f02a9e47b09e132e.rlib(common.obj) : warning LNK4217: locally defined symbol calloc imported in function lzma_alloc_zero
          liblzma_sys-f02a9e47b09e132e.rlib(common.obj) : warning LNK4217: locally defined symbol free imported in function lzma_end
          liblzma_sys-f02a9e47b09e132e.rlib(common.obj) : warning LNK4217: locally defined symbol malloc imported in function lzma_alloc
          liblzma_sys-f02a9e47b09e132e.rlib(lz_encoder.obj) : warning LNK4217: locally defined symbol memmove imported in function fill_window
          liblzma_sys-f02a9e47b09e132e.rlib(lzma_decoder.obj) : warning LNK4049: locally defined symbol memmove imported
          liblzma_sys-f02a9e47b09e132e.rlib(simple_coder.obj) : warning LNK4049: locally defined symbol memmove imported
          liblzma_sys-f02a9e47b09e132e.rlib(stream_encoder_mt.obj) : error LNK2019: unresolved external symbol __imp__beginthreadex referenced in function initialize_new_thread
          C:\projects\rust\build\x86_64-pc-windows-msvc\stage0-tools\x86_64-pc-windows-msvc\release\deps\rust_installer-32c88478981b650b.exe : fatal error LNK1120: 1 unresolved externals
          
error: aborting due to previous error
error: Could not compile `installer`.
To learn more, run the command again with --verbose.
