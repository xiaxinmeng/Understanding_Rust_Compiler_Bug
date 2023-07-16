plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0252]: the name `Object` is defined multiple times
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:9:72
  |
7 | use object::write::{self, Object, StandardSegment, Symbol, SymbolSection};
  |                           ------ previous import of the type `Object` here
8 | use object::{
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection,
  |                                                                        ^^^^^^ `Object` reimported here
  |
  = note: `Object` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
  |
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object as OtherObject, ObjectSection,

error[E0432]: unresolved import `object::xcoff`
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:9:14
  |
  |
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection,
  |              |
  |              |
  |              no `xcoff` in the root
  |              help: a similar name exists in the module: `coff`
error[E0573]: expected type, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:383:40
    |
    |
383 | pub fn create_rmeta_file_xcoff(Object: file, data: &[u8]) -> (Vec<u8>, MetadataPosition) {
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
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:384:13
    |
384 |     assert!(file.format() == BinaryFormat::Xcoff);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:386:9
    |
386 |         file.segment_name(StandardSegment::Text).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:391:9
    |
391 |         file.segment_name(StandardSegment::Data).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:396:9
    |
396 |         file.segment_name(StandardSegment::Debug).to_vec(),
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:400:5
    |
400 |     file.add_file_symbol("lib.rmeta".into());
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:401:5
    |
401 |     file.section_mut(section).flags = SectionFlags::Xcoff { s_flags: (xcoff::STYP_INFO) as u32 };
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:427:5
    |
427 |     file.append_section_data(section, &len.to_be_bytes(), 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:429:5
    |
429 |     file.append_section_data(data_section, &len.to_be_bytes(), 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
3   | use rustc_span::sym::file;
    |

error[E0423]: expected value, found macro `file`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:431:5
    |
431 |     file.append_section_data(section, data, 1);
    |
help: consider importing this constant instead
    |
3   | use rustc_span::sym::file;
---

error: unused import: `Object`
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:7:27
  |
7 | use object::write::{self, Object, StandardSegment, Symbol, SymbolSection};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `Object`
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:9:72
  |
  |
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection,

error[E0599]: no method named `section_by_name` found for struct `File` in the current scope
  --> compiler/rustc_codegen_ssa/src/back/metadata.rs:98:10
   |
   |
98 |     file.section_by_name(section)
   |          ^^^^^^^^^^^^^^^ method not found in `File<'_, &[u8]>`
  ::: /cargo/registry/src/index.crates.io-6f17d22bba15001f/object-0.30.1/src/read/traits.rs:90:8
   |
   |
90 |     fn section_by_name(&'file self, section_name: &str) -> Option<Self::Section> {
   |        --------------- the method is available for `object::File<'_, &[u8]>` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
3  | use object::Object;
3  | use object::Object;
   |

error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:300:17
    |
300 |         return (create_rmeta_file_xcoff(file, data), MetadataPosition::First);
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found tuple
    = note: expected struct `Vec<u8>`
    = note: expected struct `Vec<u8>`
                found tuple `(Vec<u8>, MetadataPosition)`
error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:347:16
    |
340 | ) -> Vec<u8> {
340 | ) -> Vec<u8> {
    |      ------- expected `Vec<u8>` because of return type
...
347 |         return create_rmeta_file_xcoff(file, &compressed);
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Vec`, found tuple
    = note: expected struct `Vec<u8>`
    = note: expected struct `Vec<u8>`
                found tuple `(Vec<u8>, MetadataPosition)`

error[E0599]: no variant named `Xcoff` found for enum `SymbolFlags<_>`
   --> compiler/rustc_codegen_ssa/src/back/metadata.rs:412:29
    |
412 |         flags: SymbolFlags::Xcoff { n_sclass : xcoff::C_INFO },
    |                             ^^^^^ variant not found in `SymbolFlags<_>`
error: unused import: `ObjectSection`
 --> compiler/rustc_codegen_ssa/src/back/metadata.rs:9:80
  |
  |
9 |     elf, pe, xcoff, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection,

Some errors have detailed explanations: E0252, E0308, E0423, E0432, E0573, E0599.
For more information about an error, try `rustc --explain E0252`.
error: could not compile `rustc_codegen_ssa` due to 25 previous errors
