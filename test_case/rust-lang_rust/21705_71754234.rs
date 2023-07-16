
    print_type_of(&12);
    print_type_of(&12i64);

    print_type_of(&12.0);
    print_type_of(&12.0f32);

    print_type_of(&"Test");
    print_type_of(&"Test".as_slice());
    print_type_of(&"Test".to_string());

    print_type_of(&(1i32..2));

    print_type_of(&[1,2,3]);
    print_type_of(&vec![1,2,3,4]);

    print_type_of(&main);
    print_type_of(&print_type_of::<isize>);

    print_type_of(&|:| 1);
    print_type_of(&|&:| 1);
    print_type_of(&|&mut:| 1);

    print_type_of(&|: x: i32| x);
