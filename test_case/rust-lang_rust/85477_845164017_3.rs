
fn baz<I: Foo<A=Bar>>(x: &<I as Foo>::A) {} // ok!
