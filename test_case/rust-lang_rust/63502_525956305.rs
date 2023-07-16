
    let fut = async {
        let x: Result<_, Error> = make_unit();
        x?;
        Ok(())
    };
