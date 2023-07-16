
    let n = Name(String::new());
    let m = Name(String::new());
    let consume = move || {
        let m = m;
        let Name(i) = n;
    };
