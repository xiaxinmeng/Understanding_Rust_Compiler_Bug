
$ rg -i '(oibit|opt-?in,?[ |_]?built-?in)' -g '!src/tools/rust-analyzer'
src/test/ui/phantom-oibit.stderr:  --> $DIR/phantom-oibit.rs:21:12
src/test/ui/phantom-oibit.stderr:  --> $DIR/phantom-oibit.rs:26:12
src/test/ui/phantom-oibit.rs:// Ensure that OIBIT checks `T` when it encounters a `PhantomData<T>` field, instead of checking
src/test/ui/traits/traits-inductive-overflow-supertrait-oibit.rs:// OIBIT-based version of #29859, supertrait version. Test that using
src/test/ui/traits/traits-inductive-overflow-supertrait-oibit.rs:// a simple OIBIT `..` impl alone still doesn't allow arbitrary bounds
src/test/ui/traits/traits-inductive-overflow-supertrait-oibit.stderr:  --> $DIR/traits-inductive-overflow-supertrait-oibit.rs:8:19
src/test/ui/traits/traits-inductive-overflow-supertrait-oibit.stderr:  --> $DIR/traits-inductive-overflow-supertrait-oibit.rs:16:23
src/test/rustdoc/auxiliary/rustdoc-impl-parts-crosscrate.rs:pub auto trait AnOibit {}
src/test/rustdoc/impl-parts.rs:pub auto trait AnOibit {}
src/test/rustdoc/impl-parts.rs://     "impl<T: Clone> !AnOibit for Foo<T> where T: Sync,"
src/test/rustdoc/impl-parts.rs:// @has impl_parts/trait.AnOibit.html '//*[@class="item-list"]//code' \
src/test/rustdoc/impl-parts.rs://     "impl<T: Clone> !AnOibit for Foo<T> where T: Sync,"
src/test/rustdoc/impl-parts.rs:impl<T: Clone> !AnOibit for Foo<T> where T: Sync {}
src/test/rustdoc/impl-parts-crosscrate.rs:// @has implementors/rustdoc_impl_parts_crosscrate/trait.AnOibit.js Bar
src/test/rustdoc/impl-parts-crosscrate.rs:// @has - !AnOibit
src/test/rustdoc/impl-parts-crosscrate.rs:impl<T: Send> !rustdoc_impl_parts_crosscrate::AnOibit for Bar<T>
compiler/rustc_middle/src/mir/mod.rs:    /// therefore don't affect the OIBIT or outlives properties of the
compiler/rustc_feature/src/removed.rs:    /// Allows features specific to OIBIT (now called auto traits).
compiler/rustc_feature/src/removed.rs:    (removed, optin_builtin_traits, "1.0.0", Some(13231), None,
compiler/rustc_feature/src/active.rs:    /// Renamed from `optin_builtin_traits`.
compiler/rustc_middle/src/traits/mod.rs:    /// There are some exceptions, e.g., around OIBITS and
compiler/rustc_mir_build/src/build/expr/as_rvalue.rs:                // and therefore is not considered during generator OIBIT
compiler/rustc_error_codes/src/error_codes/E0380.md:the [opt-in builtin traits RFC][RFC 19].
compiler/rustc_error_codes/src/error_codes/E0192.md:information see the [opt-in builtin traits RFC][RFC 19].
RELEASES.md:* The final bits of [OIBIT landed][oibit-final], meaning that traits
RELEASES.md:[oibit-final]: https://github.com/rust-lang/rust/pull/21689
RELEASES.md:      footguns and are collectively known as [opt-in built-in
RELEASES.md:      traits][oibit] (though `Sync` and `Send` will soon become pure
RELEASES.md:[oibit]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
library/rtstartup/rsbegin.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
library/rtstartup/rsend.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
library/proc_macro/src/lib.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
library/std/src/lib.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
library/core/src/lib.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
library/alloc/src/lib.rs:#![cfg_attr(bootstrap, feature(optin_builtin_traits))]
