plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0744]: `for` is not allowed in a `const fn`
     |
980  | /     for i in 0..N {
981  | |         match iter.next() {
981  | |         match iter.next() {
982  | |             Some(item_rslt) => {
983  | |                 let item = match item_rslt.branch() {
1005 | |         }
1006 | |     }
     | |_____^


error[E0277]: the trait bound `&mut F: ~const FnOnce<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
110 |         self.iter.next().map(&mut self.f)
    |                          --- ^^^^^^^^^^^ expected an `FnOnce<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |                          required by a bound introduced by this call
    |
    |
    = help: the trait `~const FnOnce<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnOnce<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
110 |         self.iter.next().map(&mut self.f)
note: required by a bound in `Option::<T>::map`
   --> library/core/src/option.rs:921:12
    |
    |
919 |     pub const fn map<U, F>(self, f: F) -> Option<U>
920 |     where
920 |     where
921 |         F: ~const FnOnce(T) -> U,
    |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
    |                                                                   ^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
note: required by a bound in `map_try_fold`
   --> library/core/src/iter/adapters/map.rs:92:8
    |
    |
90  | const fn map_try_fold<F, G, T, B, Acc, R>(x: &mut (F, G), (acc, elt): (Acc, T)) -> R
91  | where
91  | where
92  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_try_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
    |                                  ^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
note: required by a bound in `map_try_fold`
   --> library/core/src/iter/adapters/map.rs:92:8
    |
    |
90  | const fn map_try_fold<F, G, T, B, Acc, R>(x: &mut (F, G), (acc, elt): (Acc, T)) -> R
91  | where
91  | where
92  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_try_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
note: required by a bound in `map_try_fold`
   --> library/core/src/iter/adapters/map.rs:92:8
    |
    |
90  | const fn map_try_fold<F, G, T, B, Acc, R>(x: &mut (F, G), (acc, elt): (Acc, T)) -> R
91  | where
91  | where
92  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_try_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
125 |         self.iter.try_fold(init, ConstFnMutClosure::new(&mut tup, map_try_fold))
note: required by a bound in `map_try_fold`
   --> library/core/src/iter/adapters/map.rs:92:8
    |
    |
90  | const fn map_try_fold<F, G, T, B, Acc, R>(x: &mut (F, G), (acc, elt): (Acc, T)) -> R
91  | where
91  | where
92  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_try_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
    |                                                               ^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
note: required by a bound in `map_fold`
   --> library/core/src/iter/adapters/map.rs:84:8
    |
    |
82  | const fn map_fold<F, G, T, B, Acc>(x: &mut (F, G), (acc, elt): (Acc, T)) -> Acc
83  | where
83  | where
84  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
    |                              ^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
note: required by a bound in `map_fold`
   --> library/core/src/iter/adapters/map.rs:84:8
    |
    |
82  | const fn map_fold<F, G, T, B, Acc>(x: &mut (F, G), (acc, elt): (Acc, T)) -> Acc
83  | where
83  | where
84  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
note: required by a bound in `map_fold`
   --> library/core/src/iter/adapters/map.rs:84:8
    |
    |
82  | const fn map_fold<F, G, T, B, Acc>(x: &mut (F, G), (acc, elt): (Acc, T)) -> Acc
83  | where
83  | where
84  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_fold`

error[E0277]: the trait bound `&mut F: ~const FnMut<(<I as Iterator>::Item,)>` is not satisfied
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(<I as Iterator>::Item,)>` closure, found `&mut F`
    |
    = help: the trait `~const FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`
note: the trait `FnMut<(<I as Iterator>::Item,)>` is implemented for `&mut F`, but that implementation is not `const`
    |
    |
134 |         self.iter.fold(init, ConstFnMutClosure::new(&mut tup, map_fold))
note: required by a bound in `map_fold`
   --> library/core/src/iter/adapters/map.rs:84:8
    |
    |
82  | const fn map_fold<F, G, T, B, Acc>(x: &mut (F, G), (acc, elt): (Acc, T)) -> Acc
83  | where
83  | where
84  |     F: ~const FnMut(T) -> B,
    |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `map_fold`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
     |                                                        ^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
note: required by a bound in `is_false`
    --> library/core/src/iter/traits/iterator.rs:2129:30
     |
     |
2129 |         const fn is_false<F: ~const FnMut(&T) -> bool, T>(
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_false`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
     |                     ^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
note: required by a bound in `is_false`
    --> library/core/src/iter/traits/iterator.rs:2129:30
     |
     |
2129 |         const fn is_false<F: ~const FnMut(&T) -> bool, T>(
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_false`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2148 |         let mut f = ConstFnMutClosure::new(&mut tuple, is_false);
note: required by a bound in `is_false`
    --> library/core/src/iter/traits/iterator.rs:2129:30
     |
     |
2129 |         const fn is_false<F: ~const FnMut(&T) -> bool, T>(
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_false`

error[E0277]: the trait bound `for<'b> &mut ConstFnMutClosure<&mut (&mut P, usize), for<'b, 'c, 'd> fn(&'b mut (&mut P, usize), (&'c &'d mut T,)) -> bool {is_false::<&mut P, T>}>: ~const FnMut<(&'b &'a mut T,)>` is not satisfied
     |
     |
2149 |         while let Some(head) = self.find(&mut f) {
     |                                     ---- -^^^^^
     |                                     |    |
     |                                     |    expected an `FnMut<(&&'a mut T,)>` closure, found `&mut ConstFnMutClosure<&mut (&mut P, usize), for<'a, 'b, 'c> fn(&'a mut (&mut P, usize), (&'b &'c mut T,)) -> bool {is_false::<&mut P, T>}>`
     |                                     |    help: consider removing the leading `&`-reference
     |
     |
     = help: the trait `for<'b> ~const FnMut<(&'b &'a mut T,)>` is not implemented for `&mut ConstFnMutClosure<&mut (&mut P, usize), for<'a, 'b, 'c> fn(&'a mut (&mut P, usize), (&'b &'c mut T,)) -> bool {is_false::<&mut P, T>}>`
note: the trait `for<'b> FnMut<(&'b &'a mut T,)>` is implemented for `&mut ConstFnMutClosure<&mut (&mut P, usize), for<'a, 'b, 'c> fn(&'a mut (&mut P, usize), (&'b &'c mut T,)) -> bool {is_false::<&mut P, T>}>`, but that implementation is not `const`
     |
     |
2149 |         while let Some(head) = self.find(&mut f) {
note: required by a bound in `Iterator::find`
    --> library/core/src/iter/traits/iterator.rs:2744:12
     |
     |
2741 |     fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
...
...
2744 |         P: ~const FnMut(&Self::Item) -> bool + ~const Destruct,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::find`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2149 |         while let Some(head) = self.find(&mut f) {
     |                                ^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2149 |         while let Some(head) = self.find(&mut f) {
note: required by a bound in `is_false`
    --> library/core/src/iter/traits/iterator.rs:2129:30
     |
     |
2129 |         const fn is_false<F: ~const FnMut(&T) -> bool, T>(
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_false`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
     |                                                                                  ^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
note: required by a bound in `is_true`
    --> library/core/src/iter/traits/iterator.rs:2139:29
     |
     |
2139 |         const fn is_true<F: ~const FnMut(&T) -> bool, T>(
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_true`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
     |                                            ^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
note: required by a bound in `is_true`
    --> library/core/src/iter/traits/iterator.rs:2139:29
     |
     |
2139 |         const fn is_true<F: ~const FnMut(&T) -> bool, T>(
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_true`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
     |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
note: required by a bound in `is_true`
    --> library/core/src/iter/traits/iterator.rs:2139:29
     |
     |
2139 |         const fn is_true<F: ~const FnMut(&T) -> bool, T>(
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_true`

error[E0277]: the trait bound `for<'a> &mut P: ~const FnMut<(&'a T,)>` is not satisfied
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&T,)>` closure, found `&mut P`
     |
     = help: the trait `for<'a> ~const FnMut<(&'a T,)>` is not implemented for `&mut P`
note: the trait `for<'a> FnMut<(&'a T,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2150 |             if let Some(tail) = self.rfind(ConstFnMutClosure::new(&mut f.data.0, is_true)) {
note: required by a bound in `is_true`
    --> library/core/src/iter/traits/iterator.rs:2139:29
     |
     |
2139 |         const fn is_true<F: ~const FnMut(&T) -> bool, T>(
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_true`

error[E0277]: the trait bound `&mut P: ~const FnMut<(<Self as Iterator>::Item,)>` is not satisfied
     |
     |
2186 |         self.all(&mut predicate) || !self.any(predicate)
     |              --- -^^^^^^^^^^^^^
     |              |   |
     |              |   expected an `FnMut<(<Self as Iterator>::Item,)>` closure, found `&mut P`
     |              |   help: consider removing the leading `&`-reference
     |
     |
     = help: the trait `~const FnMut<(<Self as Iterator>::Item,)>` is not implemented for `&mut P`
note: the trait `FnMut<(<Self as Iterator>::Item,)>` is implemented for `&mut P`, but that implementation is not `const`
     |
     |
2186 |         self.all(&mut predicate) || !self.any(predicate)
note: required by a bound in `Iterator::all`
    --> library/core/src/iter/traits/iterator.rs:2629:12
     |
     |
2626 |     fn all<F>(&mut self, mut f: F) -> bool
...
...
2629 |         F: ~const FnMut(Self::Item) -> bool + ~const Destruct,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::all`

error[E0277]: the trait bound `for<'a, 'b> &mut F: ~const FnOnce<(&'a T, &'b T)>` is not satisfied
     |
     |
3157 |             cmp::max_by(x, y, compare)
     |             -----------       ^^^^^^^ expected an `FnOnce<(&T, &T)>` closure, found `&mut F`
     |             required by a bound introduced by this call
     |
     |
     = help: the trait `for<'a, 'b> ~const FnOnce<(&'a T, &'b T)>` is not implemented for `&mut F`
note: the trait `for<'a, 'b> FnOnce<(&'a T, &'b T)>` is implemented for `&mut F`, but that implementation is not `const`
     |
     |
3157 |             cmp::max_by(x, y, compare)
note: required by a bound in `max_by`
    --> library/core/src/cmp.rs:1328:27
     |
     |
1328 | pub const fn max_by<T, F: ~const FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `max_by`

error[E0277]: the trait bound `for<'a, 'b> &mut F: ~const FnOnce<(&'a T, &'b T)>` is not satisfied
     |
     |
3225 |             cmp::min_by(x, y, compare)
     |             -----------       ^^^^^^^ expected an `FnOnce<(&T, &T)>` closure, found `&mut F`
     |             required by a bound introduced by this call
     |
     |
     = help: the trait `for<'a, 'b> ~const FnOnce<(&'a T, &'b T)>` is not implemented for `&mut F`
note: the trait `for<'a, 'b> FnOnce<(&'a T, &'b T)>` is implemented for `&mut F`, but that implementation is not `const`
     |
     |
3225 |             cmp::min_by(x, y, compare)
note: required by a bound in `min_by`
    --> library/core/src/cmp.rs:1244:27
     |
     |
1244 | pub const fn min_by<T, F: ~const FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `min_by`

error[E0277]: the trait bound `&mut ConstFnMutClosure<&mut (B, F), for<'a> fn(&'a mut (B, F), (<A as Iterator>::Item,)) -> ControlFlow<ControlFlow<T, cmp::Ordering>> {iter_compare::compare::<B, F, <A as Iterator>::Item, T>}>: ~const FnMut<(<A as Iterator>::Item,)>` is not satisfied
     |
     |
4041 |     match a.try_for_each(&mut f) {
     |             ------------ -^^^^^
     |             |            |
     |             |            expected an `FnMut<(<A as Iterator>::Item,)>` closure, found `&mut ConstFnMutClosure<&mut (B, F), for<'a> fn(&'a mut (B, F), (<A as Iterator>::Item,)) -> ControlFlow<ControlFlow<T, cmp::Ordering>> {iter_compare::compare::<B, F, <A as Iterator>::Item, T>}>`
     |             |            help: consider removing the leading `&`-reference
     |
     |
     = help: the trait `~const FnMut<(<A as Iterator>::Item,)>` is not implemented for `&mut ConstFnMutClosure<&mut (B, F), for<'a> fn(&'a mut (B, F), (<A as Iterator>::Item,)) -> ControlFlow<ControlFlow<T, cmp::Ordering>> {iter_compare::compare::<B, F, <A as Iterator>::Item, T>}>`
note: the trait `FnMut<(<A as Iterator>::Item,)>` is implemented for `&mut ConstFnMutClosure<&mut (B, F), for<'a> fn(&'a mut (B, F), (<A as Iterator>::Item,)) -> ControlFlow<ControlFlow<T, cmp::Ordering>> {iter_compare::compare::<B, F, <A as Iterator>::Item, T>}>`, but that implementation is not `const`
     |
     |
4041 |     match a.try_for_each(&mut f) {
note: required by a bound in `Iterator::try_for_each`
    --> library/core/src/iter/traits/iterator.rs:2337:12
     |
     |
2334 |     fn try_for_each<F, R>(&mut self, mut f: F) -> R
...
...
2337 |         F: ~const FnMut(Self::Item) -> R + ~const Destruct,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_for_each`
Some errors have detailed explanations: E0277, E0744.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 23 previous errors
Build completed unsuccessfully in 0:04:06
