
[00:07:46] error[E0531]: cannot find tuple struct/variant `VtableDefaultImpl` in this scope
[00:07:46]    --> /checkout/src/librustc/ich/impls_ty.rs:859:14
[00:07:46]     |
[00:07:46] 859 |             &VtableDefaultImpl(ref table_def_impl) => table_def_impl.hash_stable(hcx, hasher),
[00:07:46]     |              ^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:46] 
[00:07:46] error[E0412]: cannot find type `VtableDefaultImplData` in module `traits`
[00:07:46]    --> /checkout/src/librustc/ich/impls_ty.rs:887:13
[00:07:46]     |
[00:07:46] 887 | for traits::VtableDefaultImplData<N> where N: HashStable<StableHashingContext<'gcx>> {
[00:07:46]     |             ^^^^^^^^^^^^^^^^^^^^^ did you mean `VtableAutoImplData`?
[00:07:46] 
[00:07:46] error[E0422]: cannot find struct, variant or union type `VtableDefaultImplData` in module `traits`
[00:07:46]    --> /checkout/src/librustc/ich/impls_ty.rs:891:21
[00:07:46]     |
[00:07:46] 891 |         let traits::VtableDefaultImplData {
[00:07:46]     |                     ^^^^^^^^^^^^^^^^^^^^^ did you mean `VtableAutoImplData`?
[00:07:46] 
[00:07:49] error[E0207]: the type parameter `N` is not constrained by the impl trait, self type, or predicates
[00:07:49]    --> /checkout/src/librustc/ich/impls_ty.rs:886:12
[00:07:49]     |
[00:07:49] 886 | impl<'gcx, N> HashStable<StableHashingContext<'gcx>>
[00:07:49]     |            ^ unconstrained type parameter
