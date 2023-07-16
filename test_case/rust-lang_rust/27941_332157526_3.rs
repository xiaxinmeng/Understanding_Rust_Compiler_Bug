
(sized_abstract, vtable) = coerce_sized self
fn_ptr = vtable[INDEX]
ret = fn_ptr(sized_abstract, ..χ)
