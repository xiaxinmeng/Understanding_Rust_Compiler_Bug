
var0 = const 0;
tmp0 = &var0;
// ... could be all sorts of stuff, including casts tmp0 to a raw pointer to potentially further inhibit analysis.
var0 = const 1;
// does not mean you can replace var0 with const 1 here, because the pointer taken above could be aliased and thus invalidates the transformation like this by all sorts of operations.
