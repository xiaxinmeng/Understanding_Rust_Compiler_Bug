plain
   Compiling num-integer v0.1.43
   Compiling tendril v0.4.3
   Compiling phf v0.10.1
   Compiling percent-encoding v2.1.0
error[E0422]: cannot find struct, variant or union type `LineColumn` in crate `proc_macro`
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.53/src/wrapper.rs:475:33
    |
475 |                 let proc_macro::LineColumn { line, column } = s.start();
    |                                 ^^^^^^^^^^ not found in `proc_macro`
help: consider importing this struct
    |
1   + use crate::LineColumn;
    |
    |
help: if you import `LineColumn`, refer to it directly
    |
475 -                 let proc_macro::LineColumn { line, column } = s.start();
475 +                 let LineColumn { line, column } = s.start();


error[E0422]: cannot find struct, variant or union type `LineColumn` in crate `proc_macro`
   --> /cargo/registry/src/index.crates.io-6f17d22bba15001f/proc-macro2-1.0.53/src/wrapper.rs:489:33
    |
489 |                 let proc_macro::LineColumn { line, column } = s.end();
    |                                 ^^^^^^^^^^ not found in `proc_macro`
help: consider importing this struct
    |
1   + use crate::LineColumn;
    |
    |
help: if you import `LineColumn`, refer to it directly
    |
489 -                 let proc_macro::LineColumn { line, column } = s.end();
489 +                 let LineColumn { line, column } = s.end();

   Compiling clap_lex v0.4.1
   Compiling strsim v0.10.0
   Compiling unicase v2.6.0
