 rust
let x = DerefExample { value: "foobar" };
    let y = MyStruct::<DerefExample<&'static str>> { var: x };
    y.dosomething();
