
$ fgrep -n -C 3 .debug_info test/run-pass/arith-0-boot.s
238-                                        # End of file scope inline assembly
239-    .section    .debug_frame,"",@progbits
240-.Lsection_debug_frame:
241:    .section    .debug_info,"",@progbits
242-.Lsection_info:
243-    .section    .debug_abbrev,"",@progbits
244-.Lsection_abbrev:
--
505-    .uleb128    2               # Offset
506-    .align  4
507-.Ldebug_frame_end1:
508:    .section    .debug_info,"",@progbits
509-.Linfo_begin1:
510-    .long   155                     # Length of Compilation Unit Info
511-    .short  2                       # DWARF version number
