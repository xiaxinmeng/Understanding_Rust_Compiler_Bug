plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/iter/adapters/map_while.rs:73:29
    |
68  |     fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
    |                  ---- this type parameter
...
73  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |                                                              ||
    |                                                              ||
    |                                                              |expected `&mut _`, found type parameter `Fold`
    |                                                              an argument of type `(_, _)` is missing
    = note: expected mutable reference `&mut _`
                  found type parameter `Fold`
note: associated function defined here
   --> library/core/src/ops/try_trait.rs:464:18
   --> library/core/src/ops/try_trait.rs:464:18
    |
464 |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465 |         f: &mut F,
    |         ---------
    |         ---------
466 |         (a, b): (A, B),
help: consider mutably borrowing here
    |
    |
73  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(&mut fold)).0
help: provide the argument
    |
    |
73  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(Acc, B)>` closure, found `NeverShortCircuit<_>`
     |
     |
73   |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
     |              --------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(Acc, B)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(Acc, B)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `Iterator::try_fold`
     |
     |
2230 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
2233 |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_fold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/iter/adapters/scan.rs:83:29
    |
    |
78  |     fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
    |                  ---- this type parameter
...
83  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |                                                              ||
    |                                                              ||
    |                                                              |expected `&mut _`, found type parameter `Fold`
    |                                                              an argument of type `(_, _)` is missing
    = note: expected mutable reference `&mut _`
                  found type parameter `Fold`
note: associated function defined here
   --> library/core/src/ops/try_trait.rs:464:18
   --> library/core/src/ops/try_trait.rs:464:18
    |
464 |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465 |         f: &mut F,
    |         ---------
    |         ---------
466 |         (a, b): (A, B),
help: consider mutably borrowing here
    |
    |
83  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(&mut fold)).0
help: provide the argument
    |
    |
83  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(Acc, B)>` closure, found `NeverShortCircuit<_>`
     |
     |
83   |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
     |              --------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(Acc, B)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(Acc, B)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `Iterator::try_fold`
     |
     |
2230 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
2233 |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_fold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/iter/adapters/skip.rs:213:30
    |
    |
209 |     fn rfold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
    |                   ---- this type parameter
...
213 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |                                                               ||
    |                                                               ||
    |                                                               |expected `&mut _`, found type parameter `Fold`
    |                                                               an argument of type `(_, _)` is missing
    = note: expected mutable reference `&mut _`
                  found type parameter `Fold`
note: associated function defined here
   --> library/core/src/ops/try_trait.rs:464:18
   --> library/core/src/ops/try_trait.rs:464:18
    |
464 |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465 |         f: &mut F,
    |         ---------
    |         ---------
466 |         (a, b): (A, B),
help: consider mutably borrowing here
    |
    |
213 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(&mut fold)).0
help: provide the argument
    |
    |
213 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
    |
    |
213 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |              ---------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
    |              required by a bound introduced by this call
    |
    |
    = help: the trait `FnMut<(Acc, <I as Iterator>::Item)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `DoubleEndedIterator::try_rfold`
    |
    |
221 |     fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
224 |         F: FnMut(B, Self::Item) -> R,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DoubleEndedIterator::try_rfold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/iter/adapters/take.rs:107:29
    |
    |
102 |     fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
    |                  ---- this type parameter
...
107 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |                                                              ||
    |                                                              ||
    |                                                              |expected `&mut _`, found type parameter `Fold`
    |                                                              an argument of type `(_, _)` is missing
    = note: expected mutable reference `&mut _`
                  found type parameter `Fold`
note: associated function defined here
   --> library/core/src/ops/try_trait.rs:464:18
   --> library/core/src/ops/try_trait.rs:464:18
    |
464 |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465 |         f: &mut F,
    |         ---------
    |         ---------
466 |         (a, b): (A, B),
help: consider mutably borrowing here
    |
    |
107 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(&mut fold)).0
help: provide the argument
    |
    |
107 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
    --> library/core/src/iter/adapters/take.rs:107:29
     |
107  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
     |              --------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(Acc, <I as Iterator>::Item)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `Iterator::try_fold`
     |
     |
2230 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
2233 |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_fold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> library/core/src/iter/adapters/take_while.rs:103:29
    |
    |
98  |     fn fold<Acc, Fold>(mut self, init: Acc, fold: Fold) -> Acc
    |                  ---- this type parameter
...
103 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
    |                                                              ||
    |                                                              ||
    |                                                              |expected `&mut _`, found type parameter `Fold`
    |                                                              an argument of type `(_, _)` is missing
    = note: expected mutable reference `&mut _`
                  found type parameter `Fold`
note: associated function defined here
   --> library/core/src/ops/try_trait.rs:464:18
   --> library/core/src/ops/try_trait.rs:464:18
    |
464 |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465 |         f: &mut F,
    |         ---------
    |         ---------
466 |         (a, b): (A, B),
help: consider mutably borrowing here
    |
    |
103 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(&mut fold)).0
help: provide the argument
    |
    |
103 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
    --> library/core/src/iter/adapters/take_while.rs:103:29
     |
103  |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(fold)).0
     |              --------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(Acc, <I as Iterator>::Item)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(Acc, <I as Iterator>::Item)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `Iterator::try_fold`
     |
     |
2230 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
2233 |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_fold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> library/core/src/iter/range.rs:1159:29
     |
     |
1154 |     fn fold<B, F>(mut self, init: B, f: F) -> B
     |                - this type parameter
...
1159 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(f)).0
     |                                                              ||
     |                                                              ||
     |                                                              |expected `&mut _`, found type parameter `F`
     |                                                              an argument of type `(_, _)` is missing
     = note: expected mutable reference `&mut _`
                   found type parameter `F`
note: associated function defined here
    --> library/core/src/ops/try_trait.rs:464:18
    --> library/core/src/ops/try_trait.rs:464:18
     |
464  |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465  |         f: &mut F,
     |         ---------
     |         ---------
466  |         (a, b): (A, B),
help: consider mutably borrowing here
     |
     |
1159 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(&mut f)).0
help: provide the argument
     |
     |
1159 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(B, A)>` closure, found `NeverShortCircuit<_>`
     |
     |
1159 |         self.try_fold(init, NeverShortCircuit::wrap_mut_2_imp(f)).0
     |              --------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(B, A)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(B, A)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `Iterator::try_fold`
     |
     |
2230 |     fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
2233 |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::try_fold`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
    --> library/core/src/iter/range.rs:1234:30
     |
     |
1229 |     fn rfold<B, F>(mut self, init: B, f: F) -> B
     |                 - this type parameter
...
1234 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(f)).0
     |                                                               ||
     |                                                               ||
     |                                                               |expected `&mut _`, found type parameter `F`
     |                                                               an argument of type `(_, _)` is missing
     = note: expected mutable reference `&mut _`
                   found type parameter `F`
note: associated function defined here
    --> library/core/src/ops/try_trait.rs:464:18
    --> library/core/src/ops/try_trait.rs:464:18
     |
464  |     pub const fn wrap_mut_2_imp<A, B, F: ~const FnMut(A, B) -> T>(
465  |         f: &mut F,
     |         ---------
     |         ---------
466  |         (a, b): (A, B),
help: consider mutably borrowing here
     |
     |
1234 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(&mut f)).0
help: provide the argument
     |
     |
1234 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(/* value */, /* value */)).0


error[E0277]: expected a `FnMut<(B, A)>` closure, found `NeverShortCircuit<_>`
     |
     |
1234 |         self.try_rfold(init, NeverShortCircuit::wrap_mut_2_imp(f)).0
     |              ---------       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(B, A)>` closure, found `NeverShortCircuit<_>`
     |              required by a bound introduced by this call
     |
     |
     = help: the trait `FnMut<(B, A)>` is not implemented for `NeverShortCircuit<_>`
note: required by a bound in `DoubleEndedIterator::try_rfold`
     |
     |
221  |     fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
...
...
224  |         F: FnMut(B, Self::Item) -> R,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `DoubleEndedIterator::try_rfold`
Some errors have detailed explanations: E0061, E0277.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `core` due to 14 previous errors
Build completed unsuccessfully in 0:01:15
