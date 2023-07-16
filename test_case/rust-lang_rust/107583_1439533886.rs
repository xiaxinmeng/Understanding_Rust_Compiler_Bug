plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0432]: unresolved import `object::xcoff`
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:9:14
  |
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection,
  |              |
  |              |
  |              no `xcoff` in the root
  |              help: a similar name exists in the module: `coff`
error[E0425]: cannot find value `bytes` in this scope
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:107:19
    |
107 |         return Ok(bytes);
107 |         return Ok(bytes);
    |                   ^^^^^ not found in this scope

error[E0573]: expected type, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:381:40
    |
381 | pub fn create_rmeta_file_xcoff(Object: file, data: &[u8]) -> (Vec<u8>, MetadataPosition) {
    |                                        |
    |                                        not a type
    |                                        not a type
    |                                        help: a struct with a similar name exists (notice the capitalization): `File`
   ::: /checkout/library/std/src/fs.rs:98:1
    |
98  | pub struct File {
    | --------------- similarly named struct `File` defined here
    | --------------- similarly named struct `File` defined here

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:382:13
    |
382 |     assert!(file.format() == BinaryFormat::Xcoff);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:384:9
    |
384 |         file.segment_name(StandardSegment::Text).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:389:9
    |
389 |         file.segment_name(StandardSegment::Data).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:394:9
    |
394 |         file.segment_name(StandardSegment::Debug).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:398:5
    |
398 |     file.add_file_symbol("lib.rmeta".into());
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:399:5
    |
399 |     file.section_mut(section).flags = SectionFlags::Xcoff { s_flags: (xcoff::STYP_INFO) as u32 };
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:425:5
    |
425 |     file.append_section_data(section, &len.to_be_bytes(), 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:427:5
    |
427 |     file.append_section_data(data_section, &len.to_be_bytes(), 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:429:5
    |
429 |     file.append_section_data(section, data, 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0599]: no method named `symbol_address_by_name` found for struct `File` in the current scope
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:110:23
    |
110 |     let offset = file.symbol_address_by_name("rustc")?;
    |                       ^^^^^^^^^^^^^^^^^^^^^^ method not found in `File<'_, &[u8]>`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:298:17
    |
    |
298 |         return (create_rmeta_file_xcoff(file, data), MetadataPosition::First);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found tuple
    = note: expected struct `Vec<u8>`
    = note: expected struct `Vec<u8>`
                found tuple `(Vec<u8>, MetadataPosition)`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:345:16
    |
338 | ) -> Vec<u8> {
338 | ) -> Vec<u8> {
    |      ------- expected `Vec<u8>` because of return type
...
345 |         return create_rmeta_file_xcoff(file, &compressed);
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found tuple
    = note: expected struct `Vec<u8>`
    = note: expected struct `Vec<u8>`
                found tuple `(Vec<u8>, MetadataPosition)`

error[E0599]: no variant named `Xcoff` found for enum `SymbolFlags<_>`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:410:29
    |
410 |         flags: SymbolFlags::Xcoff { n_sclass: xcoff::C_INFO },
    |                             ^^^^^ variant not found in `SymbolFlags<_>`
Some errors have detailed explanations: E0308, E0423, E0425, E0432, E0573, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to 22 previous errors
warning: build failed, waiting for other jobs to finish...
