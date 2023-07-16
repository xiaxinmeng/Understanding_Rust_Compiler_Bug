
; man gcc | grep -A6 -- '-gz[^\s]*$'
       -gz[=type]
           Produce compressed debug sections in DWARF format, if that is supported.  If type is not given,
           the default type depends on the capabilities of the assembler and linker used.  type may be one of
           none (don't compress debug sections), zlib (use zlib compression in ELF gABI format), or zlib-gnu
           (use zlib compression in traditional GNU format).  If the linker doesn't support writing
           compressed debug sections, the option is rejected.  Otherwise, if the assembler does not support
           them, -gz is silently ignored when producing object files.
