plain
   Compiling addr2line v0.16.0
error[E0061]: this function takes 11 arguments but 10 arguments were supplied
   --> library/std/src/../../backtrace/src/symbolize/gimli.rs:96:21
    |
96  |         let dwarf = addr2line::Context::from_sections(
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 11 arguments
97  |             load_section(stash, &object),
    |             ----------------------------
98  |             load_section(stash, &object),
    |             ----------------------------
99  |             load_section(stash, &object),
    |             ----------------------------
100 |             load_section(stash, &object),
    |             ----------------------------
101 |             load_section(stash, &object),
    |             ----------------------------
102 |             load_section(stash, &object),
    |             ----------------------------
103 |             load_section(stash, &object),
    |             ----------------------------
104 |             load_section(stash, &object),
    |             ----------------------------
105 |             load_section(stash, &object),
    |             ----------------------------
106 |             gimli::EndianSlice::new(&[], Endian),
    |
note: associated function defined here
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/addr2line-0.16.0/src/lib.rs:154:12
    |
    |
154 |     pub fn from_sections(
    |            ^^^^^^^^^^^^^

error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:42:30
    |
42  |         let elf = Elf::parse(data).ok()?;
    |                              ^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`
    |
note: required by `object::read::elf::FileHeader::parse`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.1/src/read/elf/file.rs:453:5
    |
453 |     fn parse<'data, R: ReadRef<'data>>(data: R) -> read::Result<&'data Self> {


error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:44:45
   |
44 |         let sections = elf.sections(endian, data).ok()?;
   |                                             ^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`

error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:44:24
   |
44 |         let sections = elf.sections(endian, data).ok()?;
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.1/src/read/elf/section.rs:23:8
   |
   |
23 |     R: ReadRef<'data>,
   |        -------------- required by this bound in `object::read::elf::SectionTable`

error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:44:24
   |
44 |         let sections = elf.sections(endian, data).ok()?;
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.1/src/read/elf/section.rs:23:8
   |
   |
23 |     R: ReadRef<'data>,
   |        -------------- required by this bound in `object::read::elf::SectionTable`

error[E0599]: the method `symbols` exists for struct `object::read::elf::SectionTable<'_, FileHeader64<object::LittleEndian>, object::Bytes<'_>>`, but its trait bounds were not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:46:14
   |
46 |             .symbols(endian, data, object::elf::SHT_SYMTAB)
   |              ^^^^^^^ method cannot be called on `object::read::elf::SectionTable<'_, FileHeader64<object::LittleEndian>, object::Bytes<'_>>` due to unsatisfied trait bounds
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.1/src/read/util.rs:16:1
   |
   |
16 | pub struct Bytes<'data>(pub &'data [u8]);
   | ----------------------------------------- doesn't satisfy `object::Bytes<'_>: ReadRef`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `object::Bytes<'_>: ReadRef`

error[E0599]: the method `symbols` exists for struct `object::read::elf::SectionTable<'_, FileHeader64<object::LittleEndian>, object::Bytes<'_>>`, but its trait bounds were not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:50:18
   |
50 |                 .symbols(endian, data, object::elf::SHT_DYNSYM)
   |                  ^^^^^^^ method cannot be called on `object::read::elf::SectionTable<'_, FileHeader64<object::LittleEndian>, object::Bytes<'_>>` due to unsatisfied trait bounds
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.1/src/read/util.rs:16:1
   |
   |
16 | pub struct Bytes<'data>(pub &'data [u8]);
   | ----------------------------------------- doesn't satisfy `object::Bytes<'_>: ReadRef`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `object::Bytes<'_>: ReadRef`
error[E0308]: mismatched types
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:85:13
   |
85 |             sections,
85 |             sections,
   |             ^^^^^^^^ expected `&[u8]`, found struct `object::Bytes`
   |
   = note: expected struct `object::read::elf::SectionTable<'_, _, &[u8]>`
              found struct `object::read::elf::SectionTable<'_, _, object::Bytes<'_>>`

error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
  --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:93:54
   |
93 |             let mut data = section.data(self.endian, self.data).ok()?;
   |                                                      ^^^^^^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`
error[E0609]: no field `0` on type `&[u8]`
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:100:34
    |
100 |                 return Some(data.0);
100 |                 return Some(data.0);
    |                                  ^

error[E0599]: no method named `read` found for reference `&[u8]` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:103:31
    |
103 |             let header = data.read::<<Elf as FileHeader>::CompressionHeader>().ok()?;
    |                               ^^^^ method not found in `&[u8]`
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use object::ReadRef;`
error[E0609]: no field `0` on type `&[u8]`
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:110:34
    |
    |
110 |             decompress_zlib(data.0, buf)?;


error[E0277]: the trait bound `object::Bytes<'_>: ReadRef<'_>` is not satisfied
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:134:61
    |
134 |         let mut data = compressed_section.data(self.endian, self.data).ok()?;
    |                                                             ^^^^^^^^^ the trait `ReadRef<'_>` is not implemented for `object::Bytes<'_>`

error[E0599]: no method named `read_bytes` found for reference `&[u8]` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:135:17
    |
135 |         if data.read_bytes(8).ok()?.0 != b"ZLIB\0\0\0\0" {
    |                 ^^^^^^^^^^ method not found in `&[u8]`
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use object::ReadRef;`

error[E0599]: no method named `read` found for reference `&[u8]` in the current scope
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:138:41
    |
138 |         let size = usize::try_from(data.read::<object::U32Bytes<_>>().ok()?.get(BigEndian)).ok()?;
    |                                         ^^^^ method not found in `&[u8]`
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use object::ReadRef;`
error[E0609]: no field `0` on type `&[u8]`
   --> library/std/src/../../backtrace/src/symbolize/gimli/elf.rs:140:30
    |
    |
140 |         decompress_zlib(data.0, buf)?;

Some errors have detailed explanations: E0061, E0277, E0308, E0599, E0609.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `std` due to 16 previous errors
