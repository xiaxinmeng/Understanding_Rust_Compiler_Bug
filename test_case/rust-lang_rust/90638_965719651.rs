
a=for<'r> extern "rust-call" fn((<Y as Yokeable<'r>>::Output, &'r ())) -> std::boxed::Box<(dyn for<'r> IsCovariant<'r> + 'r)>
b=for<'r> extern "rust-call" fn((<Y as Yokeable<'r>>::Output, &'r ())) -> <std::boxed::Box<dyn IsCovariant<'_>> as Yokeable<'r>>::Output
