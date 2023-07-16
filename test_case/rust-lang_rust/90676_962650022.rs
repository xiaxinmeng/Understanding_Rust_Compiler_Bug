plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0541]: unknown meta item 'feaure'
    |
    |
186 |     #[unstable(feaure = "iter_is_empty", reason = "recently added", issue = "0")]
    |                ^^^^^^^^^^^^^^^^^^^^^^^^ expected one of `feature`, `reason`, `issue`, `soft`
error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/cloned.rs:112:17
    |
    |
112 |         self.it.is_empty()
    |                 ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
112 |         ExactSizeIterator::is_empty(self.it)
help: disambiguate the associated function for candidate #2
    |
    |
112 |         Iterator::is_empty(self.it)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/copied.rs:138:17
    |
    |
138 |         self.it.is_empty()
    |                 ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
138 |         ExactSizeIterator::is_empty(self.it)
help: disambiguate the associated function for candidate #2
    |
    |
138 |         Iterator::is_empty(self.it)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/enumerate.rs:228:19
    |
    |
228 |         self.iter.is_empty()
    |                   ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
228 |         ExactSizeIterator::is_empty(self.iter)
help: disambiguate the associated function for candidate #2
    |
    |
228 |         Iterator::is_empty(self.iter)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/fuse.rs:206:36
    |
    |
206 |             Some(ref iter) => iter.is_empty(),
    |                                    ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
206 |             Some(ref iter) => ExactSizeIterator::is_empty(&iter),
help: disambiguate the associated function for candidate #2
    |
    |
206 |             Some(ref iter) => Iterator::is_empty(&iter),

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/inspect.rs:144:19
    |
    |
144 |         self.iter.is_empty()
    |                   ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
144 |         ExactSizeIterator::is_empty(self.iter)
help: disambiguate the associated function for candidate #2
    |
    |
144 |         Iterator::is_empty(self.iter)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/map.rs:175:19
    |
    |
175 |         self.iter.is_empty()
    |                   ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
175 |         ExactSizeIterator::is_empty(self.iter)
help: disambiguate the associated function for candidate #2
    |
    |
175 |         Iterator::is_empty(self.iter)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/adapters/rev.rs:129:19
    |
    |
129 |         self.iter.is_empty()
    |                   ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
129 |         ExactSizeIterator::is_empty(self.iter)
help: disambiguate the associated function for candidate #2
    |
    |
129 |         Iterator::is_empty(self.iter)

error[E0034]: multiple applicable items in scope
   --> library/core/src/iter/traits/exact_size.rs:145:18
    |
    |
145 |         (**self).is_empty()
    |                  ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in the trait `ExactSizeIterator`
    |
    |
134 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `Iterator`
    |
    |
187 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
    |
145 |         ExactSizeIterator::is_empty((**self))
help: disambiguate the associated function for candidate #2
    |
    |
145 |         Iterator::is_empty((**self))

error[E0034]: multiple applicable items in scope
    --> library/core/src/slice/iter.rs:2225:19
     |
     |
2225 |         self.iter.is_empty()
     |                   ^^^^^^^^ multiple `is_empty` found
     |
note: candidate #1 is defined in an impl of the trait `ExactSizeIterator` for the type `slice::iter::Iter<'_, T>`
     |
41   |  / macro_rules! iterator {
42   |  |     (
42   |  |     (
43   |  |         struct $name:ident -> $ptr:ty,
44   |  |         $elem:ty,
...     |
124  |  |             fn is_empty(&self) -> bool {
...     |
397  |  |     }
398  |  | }
     |  |_- in this expansion of `iterator!`
     |  |_- in this expansion of `iterator!`
     |
    ::: library/core/src/slice/iter.rs:134:1
     |
134  | /  iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
135  | |      fn is_sorted_by<F>(self, mut compare: F) -> bool
136  | |      where
137  | |          Self: Sized,
143  | |      }
144  | |  }}
     | |___- in this macro invocation
     | |___- in this macro invocation
note: candidate #2 is defined in an impl of the trait `Iterator` for the type `slice::iter::Iter<'a, T>`
     |
     |
187  |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
     |
     |
2225 |         ExactSizeIterator::is_empty(&self.iter)
help: disambiguate the associated function for candidate #2
     |
     |
2225 |         Iterator::is_empty(&self.iter)

error[E0034]: multiple applicable items in scope
    --> library/core/src/slice/iter.rs:2343:19
     |
     |
2343 |         self.iter.is_empty()
     |                   ^^^^^^^^ multiple `is_empty` found
     |
note: candidate #1 is defined in an impl of the trait `ExactSizeIterator` for the type `slice::iter::IterMut<'_, T>`
     |
41   | / macro_rules! iterator {
42   | |     (
42   | |     (
43   | |         struct $name:ident -> $ptr:ty,
44   | |         $elem:ty,
...    |
124  | |             fn is_empty(&self) -> bool {
...    |
397  | |     }
398  | | }
     | |_- in this expansion of `iterator!`
     | |_- in this expansion of `iterator!`
     |
    ::: library/core/src/slice/iter.rs:316:1
     |
316  |   iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
     |   --------------------------------------------------------------- in this macro invocation
note: candidate #2 is defined in an impl of the trait `Iterator` for the type `slice::iter::IterMut<'a, T>`
     |
     |
187  |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
     |
     |
2343 |         ExactSizeIterator::is_empty(&self.iter)
help: disambiguate the associated function for candidate #2
     |
     |
2343 |         Iterator::is_empty(&self.iter)

error[E0034]: multiple applicable items in scope
   --> library/core/src/str/iter.rs:342:16
    |
    |
342 |         self.0.is_empty()
    |                ^^^^^^^^ multiple `is_empty` found
    |
note: candidate #1 is defined in an impl of the trait `Iterator` for the type `Copied<I>`
    |
    |
187 |     fn is_empty(&self) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `ExactSizeIterator` for the type `Copied<I>`
   --> library/core/src/iter/adapters/copied.rs:137:5
    |
137 |     fn is_empty(&self) -> bool {
help: disambiguate the associated function for candidate #1
    |
342 |         Iterator::is_empty(&self.0)
    |
