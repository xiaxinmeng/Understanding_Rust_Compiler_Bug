rust
    type OpaqueType<'a> = impl Send;
    fn convert<'a, 'b, T: ?Sized>(x: &'a T, _proof: &'b OpaqueType<'a>) -> &'b T {
        x
    }
    let r;
    {
        let x = String::from("Hello World?");
        r = convert(&x, &()); // error `x` must live for `'static`
    }
    println!("{}", r);
