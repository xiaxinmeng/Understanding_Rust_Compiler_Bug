
let r: Result<(), E> =
    do catch {
        foo()?;
        bar()?;
        qux()?;
    };
