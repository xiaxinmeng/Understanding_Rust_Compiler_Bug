compile_fail,E0412
2019-08-04T16:30:58.2376260Z - impl Something {} // error: type name `Something` is not in scope
2019-08-04T16:30:58.2376449Z - 
2019-08-04T16:30:58.2376661Z - // or:
2019-08-04T16:30:58.2377039Z - trait Foo {
2019-08-04T16:30:58.2377039Z - trait Foo {
2019-08-04T16:30:58.2377291Z -     fn bar(N); // error: type name `N` is not in scope
2019-08-04T16:30:58.2377491Z - }
2019-08-04T16:30:58.2377800Z - 
2019-08-04T16:30:58.2377983Z - // or:
2019-08-04T16:30:58.2378178Z - 
2019-08-04T16:30:58.2378399Z - fn foo(x: T) {} // type name `T` is not in scope
2019-08-04T16:30:58.2378584Z - 