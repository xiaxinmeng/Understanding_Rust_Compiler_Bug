
rust1: note: AST::InherentImpl resolve Self: {generics30::Foo::<T, f32>}
rust1: note: AST::InherentImpl resolve_impl_item: impl_prefix={generics30::Foo::<T, f32>} cpath={generics30::Foo::<T, f32>}
rust1: note: coercion_site id={46} expected={X} expr={X}
rust1: note: coerce_unsized(source={X}, target={X})
rust1: note: coerce_default_unify(a={X}, b={X})
rust1: note: unify_site id={46} expected={X} expr={X}
gcc/testsuite/rust/compile/torture/generics30.rs:12:9: note: resolved_call_expr to: {Foo<T?, T?>}
   12 |     a = Foo(123, 456f32);
      |         ^
rust1: note: coercion_site id={53} expected={T?} expr={<integer>}
rust1: note: coerce_unsized(source={<integer>}, target={T?})
rust1: note: coerce_default_unify(a={<integer>}, b={T?})
rust1: note: unify_site id={53} expected={T?} expr={<integer>}
rust1: note: coercion_site id={54} expected={T?} expr={f32}
rust1: note: coerce_unsized(source={f32}, target={T?})
rust1: note: coerce_default_unify(a={f32}, b={T?})
rust1: note: unify_site id={54} expected={T?} expr={f32}
rust1: note: coercion_site id={56} expected={T?} expr={Foo<<integer>, f32>}
rust1: note: coerce_unsized(source={Foo<<integer>, f32>}, target={T?})
rust1: note: coerce_default_unify(a={Foo<<integer>, f32>}, b={T?})
rust1: note: unify_site id={56} expected={T?} expr={Foo<<integer>, f32>}
rust1: note: autoderef try 1: {Foo<<integer>, f32>}
rust1: note: inherent_impl_fns found {1}, trait_fns found {0}, predicate_items found {0}
rust1: note: dot-operator impl_item fn_self={Foo<T, f32>} can_eq receiver={Foo<<integer>, f32>}
gcc/testsuite/rust/compile/torture/generics30.rs:15:11: note: resolved method to: {47} {fn<T, X> (self Foo<T, f32>{Foo (0:T, 1:f32)},a X=X REF: 37,) -> X=X REF: 37}
   15 |     b = a.test::<bool>(false);
      |           ^
gcc/testsuite/rust/compile/torture/generics30.rs:15:18: note: applying generic arguments to method_call: {fn<<integer>, X> (self Foo<<integer>, f32>{Foo (0:<integer>, 1:f32)},a X=X REF: 37,) -> X=X REF: 37}
   15 |     b = a.test::<bool>(false);
      |                  ^
rust1: note: type-checking method_call: {fn<<integer>, X> (self Foo<<integer>, f32>{Foo (0:<integer>, 1:f32)},a X=X REF: 37,) -> X=X REF: 37}
rust1: note: unify_site id={66} expected={Foo<<integer>, f32>} expr={Foo<<integer>, f32>}
rust1: note: coercion_site id={65} expected={X} expr={bool}
rust1: note: coerce_unsized(source={bool}, target={X})
rust1: note: coerce_default_unify(a={bool}, b={X})
rust1: note: unify_site id={65} expected={X} expr={bool}
gcc/testsuite/rust/compile/torture/generics30.rs:15:24: error: expected ‘X’ got ‘bool’
    4 |     fn test<X>(self, a: X) -> X {
      |                      ~
......
   15 |     b = a.test::<bool>(false);
      |                        ^
gcc/testsuite/rust/compile/torture/generics30.rs:15:24: error: Type Resolution failure on parameter
gcc/testsuite/rust/compile/torture/generics30.rs:15:9: error: failed to lookup type to MethodCallExpr
   15 |     b = a.test::<bool>(false);
      |         ^
gcc/testsuite/rust/compile/torture/generics30.rs:15:9: error: failed to type resolve expression
rust1: note: coercion_site id={67} expected={T?} expr={<tyty::error>}
rust1: note: coercion_site id={69} expected={()} expr={()}
rust1: note: coerce_unsized(source={()}, target={()})
rust1: note: coerce_default_unify(a={()}, b={()})
rust1: note: unify_site id={69} expected={()} expr={()}
