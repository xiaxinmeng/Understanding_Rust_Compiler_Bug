plain
   Compiling md-5 v0.9.1
   Compiling sha-1 v0.9.1
   Compiling sha2 v0.9.1
   Compiling rustc-rayon v0.3.0
error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
84  |     fn par_chunks(&self, chunk_size: usize) -> Chunks<'_, T> {
    |                                                ^^^^^^ ambiguous name
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
86  |         Chunks {
    |         ^^^^^^ ambiguous name
    |
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
510 | #[derive(Debug)]
    |          ^^^^^ ambiguous name
    |
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
516 | impl<'data, T: Sync> Clone for Chunks<'data, T> {
    |                                ^^^^^^ ambiguous name
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
518 |         Chunks { ..*self }
    |         ^^^^^^ ambiguous name
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
522 | impl<'data, T: Sync + 'data> ParallelIterator for Chunks<'data, T> {
    |                                                   ^^^^^^ ambiguous name
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Chunks` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
537 | impl<'data, T: Sync + 'data> IndexedParallelIterator for Chunks<'data, T> {
    |                                                          ^^^^^^ ambiguous name
    |
note: `Chunks` could refer to the struct defined here
    |
    |
511 | / pub struct Chunks<'data, T: 'data + Sync> {
512 | |     chunk_size: usize,
513 | |     slice: &'data [T],
    | |_^
    | |_^
note: `Chunks` could also refer to the struct imported here
    |
16  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Chunks` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
154 |     fn par_split<P: Pattern>(&self, separator: P) -> Split<'_, P> {
    |                                                      ^^^^^ ambiguous name
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
155 |         Split::new(self.as_parallel_string(), separator)
    |         ^^^^^ ambiguous name
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
582 | #[derive(Debug, Clone)]
    |          ^^^^^ ambiguous name
    |
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
582 | #[derive(Debug, Clone)]
    |                 ^^^^^ ambiguous name
    |
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
588 | impl<'ch, P: Pattern> Split<'ch, P> {
    |                       ^^^^^ ambiguous name
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
590 |         Split { chars, separator }
    |         ^^^^^ ambiguous name
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate

error[E0659]: `Split` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
    |
    |
594 | impl<'ch, P: Pattern> ParallelIterator for Split<'ch, P> {
    |                                            ^^^^^ ambiguous name
    |
note: `Split` could refer to the struct defined here
    |
    |
583 | / pub struct Split<'ch, P: Pattern> {
584 | |     chars: &'ch str,
585 | |     separator: P,
    | |_^
    | |_^
note: `Split` could also refer to the struct imported here
    |
17  | use iter::*;
    |     ^^^^^^^
    |     ^^^^^^^
    = help: consider adding an explicit import of `Split` to disambiguate
   Compiling matchers v0.0.1
   Compiling tempfile v3.1.0
error: aborting due to 14 previous errors

