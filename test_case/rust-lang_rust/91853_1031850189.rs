plain
[RUSTC-TIMING] rustc_demangle test:false 2.745
[RUSTC-TIMING] panic_abort test:false 0.125
[RUSTC-TIMING] std_detect test:false 0.289
[RUSTC-TIMING] panic_unwind test:false 0.354
error: missing recommended bound on `SegmentIterator`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:17:5
17 |     type SegmentIterator: Iterator<Item = Self::Segment>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                                         |
   |                                                         |
   |                                                         help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `Section`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:20:5
   |
   |
20 |     type Section: ObjectSection<'data>;
   |                                       |
   |                                       |
   |                                       help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `SectionIterator`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:23:5
   |
23 |     type SectionIterator: Iterator<Item = Self::Section>;
23 |     type SectionIterator: Iterator<Item = Self::Section>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                                         |
   |                                                         help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information

error: missing recommended bound on `ComdatIterator`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:29:5
   |
29 |     type ComdatIterator: Iterator<Item = Self::Comdat>;
   |                                                       |
   |                                                       |
   |                                                       help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `Symbol`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:32:5
   |
   |
32 |     type Symbol: ObjectSymbol<'data>;
   |                                     |
   |                                     |
   |                                     help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `SymbolIterator`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:35:5
   |
   |
35 |     type SymbolIterator: Iterator<Item = Self::Symbol>;
   |                                                       |
   |                                                       |
   |                                                       help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `SymbolTable`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:38:5
   |
38 | /     type SymbolTable: ObjectSymbolTable<
38 | /     type SymbolTable: ObjectSymbolTable<
39 | |         'data,
40 | |         Symbol = Self::Symbol,
41 | |         SymbolIterator = Self::SymbolIterator,
42 | |     >;
   | |      ^ help: add the recommended where clause: `where Self: 'file`
   | 
   |
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing recommended bound on `DynamicRelocationIterator`
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.26.2/src/read/traits.rs:48:5
   |
   |
48 |     type DynamicRelocationIterator: Iterator<Item = (u64, Relocation)>;
   |                                                                       |
   |                                                                       |
   |                                                                       help: add the recommended where clause: `where Self: 'file`
   |
   = note: this bound is currently recommended to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
[RUSTC-TIMING] hashbrown test:false 0.909
[RUSTC-TIMING] alloc test:false 3.617
[RUSTC-TIMING] object test:false 2.451
error: could not compile `object` due to 8 previous errors
