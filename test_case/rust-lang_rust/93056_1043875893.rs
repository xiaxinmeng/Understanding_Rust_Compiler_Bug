plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `try_map_id` found for struct `std::boxed::Box<T>` in the current scope
   --> compiler/rustc_middle/src/ty/structural_impls.rs:824:14
    |
824 |         self.try_map_id(|value| value.try_fold_with(folder))
    |              ^^^^^^^^^^ method not found in `std::boxed::Box<T>`
   ::: /checkout/compiler/rustc_data_structures/src/functor.rs:7:8
    |
    |
7   |     fn try_map_id<F, E>(self, f: F) -> Result<Self, E>
    |        ---------- the method is available for `std::boxed::Box<T>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
5   | use crate::rustc_data_structures::functor::IdFunctor;
5   | use crate::rustc_data_structures::functor::IdFunctor;
    |

error[E0599]: no method named `try_map_id` found for struct `Vec<T>` in the current scope
   --> compiler/rustc_middle/src/ty/structural_impls.rs:837:14
    |
837 |         self.try_map_id(|t| t.try_fold_with(folder))
    |              ^^^^^^^^^^ method not found in `Vec<T>`
   ::: /checkout/compiler/rustc_data_structures/src/functor.rs:7:8
    |
    |
7   |     fn try_map_id<F, E>(self, f: F) -> Result<Self, E>
    |        ---------- the method is available for `Vec<T>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
5   | use crate::rustc_data_structures::functor::IdFunctor;
5   | use crate::rustc_data_structures::functor::IdFunctor;
    |

error[E0599]: no method named `try_map_id` found for struct `std::boxed::Box<[T]>` in the current scope
   --> compiler/rustc_middle/src/ty/structural_impls.rs:850:14
    |
850 |         self.try_map_id(|t| t.try_fold_with(folder))
    |              ^^^^^^^^^^ method not found in `std::boxed::Box<[T]>`
   ::: /checkout/compiler/rustc_data_structures/src/functor.rs:7:8
    |
    |
7   |     fn try_map_id<F, E>(self, f: F) -> Result<Self, E>
    |        ---------- the method is available for `std::boxed::Box<[T]>` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
5   | use crate::rustc_data_structures::functor::IdFunctor;
5   | use crate::rustc_data_structures::functor::IdFunctor;
    |

error[E0599]: no method named `try_map_id` found for struct `rustc_index::vec::IndexVec` in the current scope
    --> compiler/rustc_middle/src/ty/structural_impls.rs:1151:14
     |
1151 |         self.try_map_id(|x| x.try_fold_with(folder))
     |              ^^^^^^^^^^ method not found in `rustc_index::vec::IndexVec<I, T>`
    ::: /checkout/compiler/rustc_data_structures/src/functor.rs:7:8
     |
     |
7    |     fn try_map_id<F, E>(self, f: F) -> Result<Self, E>
     |        ---------- the method is available for `rustc_index::vec::IndexVec<I, T>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
5    | use crate::rustc_data_structures::functor::IdFunctor;
