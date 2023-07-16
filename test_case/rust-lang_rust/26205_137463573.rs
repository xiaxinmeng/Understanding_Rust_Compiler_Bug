
DEBUG:rustc_typeck::check: autoderef(base_ty=&Bar, opt_expr=Some(expr(37: self)), lvalue_pref=NoPreference)
DEBUG:rustc_typeck::check: autoderef(base_ty=&Foo, opt_expr=Some(expr(35: &self.0)), lvalue_pref=NoPreference)
DEBUG:rustc_typeck::check: autoderef(base_ty=&mut Bar, opt_expr=Some(expr(52: self)), lvalue_pref=PreferMutLvalue)
DEBUG:rustc_typeck::check: autoderef(base_ty=&mut Foo, opt_expr=Some(expr(50: &mut self.0)), lvalue_pref=PreferMutLvalue)
DEBUG:rustc_typeck::check: autoderef(base_ty=Box<Bar>, opt_expr=None, lvalue_pref=NoPreference)
DEBUG:rustc_typeck::check: autoderef(base_ty=Box<Bar>, opt_expr=Some(expr(65: bar)), lvalue_pref=NoPreference)
DEBUG:rustc_typeck::check: autoderef(base_ty=Box<Bar>, opt_expr=Some(expr(65: bar)), lvalue_pref=PreferMutLvalue)
