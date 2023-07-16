
fn baz<I>(x: &<I as Foo>::A) where I: Foo<A=Bar> {}
