
    let mut my_stuff = std::collections::HashMap::new();
    my_stuff.insert(0i, 42i);

    let mut it = my_stuff.iter();
    my_stuff.swap(1, 43);
