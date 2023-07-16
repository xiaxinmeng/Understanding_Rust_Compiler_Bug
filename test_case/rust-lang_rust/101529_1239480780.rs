
> assert_eq!(&format!("{:+}", Foo(23)), "Foo(+23)");
> assert_eq!(&format!("{}", Foo(23)), "Foo(23)");
> // ADD Foo(-23) HERE
> 