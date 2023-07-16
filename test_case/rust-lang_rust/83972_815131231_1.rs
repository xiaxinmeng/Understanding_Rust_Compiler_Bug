
input to MirNeighborCollector::monomorphize    for<'r> fn(&'r <B as Bar>::Foo) {<<B as Bar>::Foo as Foo>::foo}
after subst_mir_and_normalize_erasing_regions  for<'r> fn(&'r FooImpl<{ 4 }>) {<FooImpl<{ 4 }> as Foo>::foo}
after an exra normalize_erasing_regions (*)    for<'r> fn(&'r FooImpl<4_usize>) {<FooImpl<4_usize> as Foo>::foo}
