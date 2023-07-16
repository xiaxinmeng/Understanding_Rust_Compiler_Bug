plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.85
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: `for` is not allowed in a `const fn`
    |
336 | /         for i in 0..n {
336 | /         for i in 0..n {
337 | |             self.next().ok_or(i)?;
    | |_________^
    |
    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
    = help: add `#![feature(const_for)]` to the crate attributes to enable
---
    |                          ^^^^^ --------- required by a bound introduced by this call
    |                          |
    |                          `U` is not an iterator
    |
note: required for `U` to implement `~const IntoIterator`
    |
    |
267 | impl<I: Iterator> const IntoIterator for I {
help: consider further restricting this bound
    |
    |
512 |         U: IntoIterator<Item = Self::Item> + iter::traits::iterator::Iterator,

error[E0277]: `U` is not an iterator
   --> library/core/src/iter/traits/iterator.rs:633:24
    |
    |
633 |         Zip::new(self, other.into_iter())
    |                        |
    |                        `U` is not an iterator
    |
    |
note: required for `U` to implement `~const IntoIterator`
    |
    |
267 | impl<I: Iterator> const IntoIterator for I {
help: consider further restricting this bound
    |
631 |         U: IntoIterator + iter::traits::iterator::Iterator,
    |                         ++++++++++++++++++++++++++++++++++
    |                         ++++++++++++++++++++++++++++++++++

error[E0277]: the trait bound `F: ~const FnMut<(B, <Self as Iterator>::Item)>` is not satisfied
     |
     |
2281 |             accum = f(accum, x)?;
     |                     ^^^^^^^^^^^ expected an `FnMut<(B, <Self as Iterator>::Item)>` closure, found `F`
     |
note: the trait `FnMut<(B, <Self as Iterator>::Item)>` is implemented for `F`, but that implementation is not `const`
     |
     |
2281 |             accum = f(accum, x)?;
help: consider further restricting this bound
     |
     |
2276 |         F: FnMut(B, Self::Item) -> R + ~const ops::function::FnMut<(B, <Self as iter::traits::iterator::Iterator>::Item)>,


error[E0277]: the trait bound `F: ~const FnMut<(B, <Self as Iterator>::Item)>` is not satisfied
     |
     |
2459 |             accum = f(accum, x);
     |                     ^^^^^^^^^^^ expected an `FnMut<(B, <Self as Iterator>::Item)>` closure, found `F`
     |
note: the trait `FnMut<(B, <Self as Iterator>::Item)>` is implemented for `F`, but that implementation is not `const`
     |
     |
2459 |             accum = f(accum, x);
help: consider further restricting this bound
     |
     |
2455 |         F: FnMut(B, Self::Item) -> B + ~const ops::function::FnMut<(B, <Self as iter::traits::iterator::Iterator>::Item)>,


error[E0277]: the trait bound `R: ~const Try` is not satisfied
     |
     |
2574 |         match self.try_fold(first, f).branch() {
     |               |
     |               |
     |               the trait `~const Try` is not implemented for `R`
     |
note: the trait `Try` is implemented for `R`, but that implementation is not `const`
     |
     |
2574 |         match self.try_fold(first, f).branch() {
help: consider further restricting this bound
     |
     |
2566 |         R: Try<Output = Self::Item> + ~const ops::try_trait::Try,


error[E0277]: can't compare `ControlFlow<()>` with `_` in const contexts
     |
     |
2631 |         self.try_fold((), check(f)) == ControlFlow::CONTINUE
     |                                     ^^ no implementation for `ControlFlow<()> == _`
     |
     = help: the trait `~const cmp::PartialEq<_>` is not implemented for `ControlFlow<()>`
note: the trait `cmp::PartialEq<_>` is implemented for `ControlFlow<()>`, but that implementation is not `const`
     |
     |
2631 |         self.try_fold((), check(f)) == ControlFlow::CONTINUE


error[E0277]: can't compare `ControlFlow<()>` with `_` in const contexts
     |
     |
2686 |         self.try_fold((), check(f)) == ControlFlow::BREAK
     |                                     ^^ no implementation for `ControlFlow<()> == _`
     |
     = help: the trait `~const cmp::PartialEq<_>` is not implemented for `ControlFlow<()>`
note: the trait `cmp::PartialEq<_>` is implemented for `ControlFlow<()>`, but that implementation is not `const`
     |
     |
2686 |         self.try_fold((), check(f)) == ControlFlow::BREAK


error[E0277]: the trait bound `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>: ~const Iterator` is not satisfied
     |
     |
3097 |         let (_, x) = self.map(key(f)).max_by(compare)?;
     |                      |
     |                      |
     |                      `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>` is not an iterator
     |
     = help: the trait `~const Iterator` is not implemented for `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>`
note: the trait `Iterator` is implemented for `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>`, but that implementation is not `const`
     |
     |
3097 |         let (_, x) = self.map(key(f)).max_by(compare)?;


error[E0277]: the trait bound `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>: ~const Iterator` is not satisfied
     |
     |
3159 |         let (_, x) = self.map(key(f)).min_by(compare)?;
     |                      |
     |                      |
     |                      `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>` is not an iterator
     |
     = help: the trait `~const Iterator` is not implemented for `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>`
note: the trait `Iterator` is implemented for `Map<Self, impl FnMut(<Self as Iterator>::Item) -> (B, <Self as Iterator>::Item)>`, but that implementation is not `const`
     |
     |
3159 |         let (_, x) = self.map(key(f)).min_by(compare)?;

error[E0277]: `I` is not an iterator
    --> library/core/src/iter/traits/iterator.rs:3545:34
     |
     |
3545 |         match iter_compare(self, other.into_iter(), compare(cmp)) {
     |                                  |
     |                                  `I` is not an iterator
     |
     |
note: required for `I` to implement `~const IntoIterator`
     |
     |
267  | impl<I: Iterator> const IntoIterator for I {
help: consider further restricting this bound
     |
3531 |         I: IntoIterator + iter::traits::iterator::Iterator,
     |                         ++++++++++++++++++++++++++++++++++
     |                         ++++++++++++++++++++++++++++++++++

error[E0277]: `I` is not an iterator
    --> library/core/src/iter/traits/iterator.rs:3623:34
     |
3623 |         match iter_compare(self, other.into_iter(), compare(partial_cmp)) {
     |                                  |
     |                                  `I` is not an iterator
     |
     |
note: required for `I` to implement `~const IntoIterator`
     |
     |
267  | impl<I: Iterator> const IntoIterator for I {
help: consider further restricting this bound
     |
3609 |         I: IntoIterator + iter::traits::iterator::Iterator,
     |                         ++++++++++++++++++++++++++++++++++
     |                         ++++++++++++++++++++++++++++++++++

error[E0277]: `I` is not an iterator
    --> library/core/src/iter/traits/iterator.rs:3682:34
     |
3682 |         match iter_compare(self, other.into_iter(), compare(eq)) {
     |                                  |
     |                                  `I` is not an iterator
     |
     |
note: required for `I` to implement `~const IntoIterator`
     |
     |
267  | impl<I: Iterator> const IntoIterator for I {
help: consider further restricting this bound
     |
3669 |         I: IntoIterator + iter::traits::iterator::Iterator,
     |                         ++++++++++++++++++++++++++++++++++
     |                         ++++++++++++++++++++++++++++++++++

error[E0277]: can't compare `Option<cmp::Ordering>` with `_` in const contexts
     |
     |
3727 |         self.partial_cmp(other) == Some(Ordering::Less)
     |                                 ^^ no implementation for `Option<cmp::Ordering> == _`
     |
     = help: the trait `~const cmp::PartialEq<_>` is not implemented for `Option<cmp::Ordering>`
note: the trait `cmp::PartialEq<_>` is implemented for `Option<cmp::Ordering>`, but that implementation is not `const`
     |
     |
3727 |         self.partial_cmp(other) == Some(Ordering::Less)


error[E0277]: can't compare `Option<cmp::Ordering>` with `_` in const contexts
     |
     |
3771 |         self.partial_cmp(other) == Some(Ordering::Greater)
     |                                 ^^ no implementation for `Option<cmp::Ordering> == _`
     |
     = help: the trait `~const cmp::PartialEq<_>` is not implemented for `Option<cmp::Ordering>`
note: the trait `cmp::PartialEq<_>` is implemented for `Option<cmp::Ordering>`, but that implementation is not `const`
     |
     |
3771 |         self.partial_cmp(other) == Some(Ordering::Greater)


error[E0277]: the trait bound `Map<Self, F>: ~const Iterator` is not satisfied
     |
     |
3901 |         self.map(f).is_sorted()
     |         |
     |         |
     |         `Map<Self, F>` is not an iterator
     |
     = help: the trait `~const Iterator` is not implemented for `Map<Self, F>`
note: the trait `Iterator` is implemented for `Map<Self, F>`, but that implementation is not `const`
     |
     |
3901 |         self.map(f).is_sorted()

Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 16 previous errors
