rust
    // initialize environment 
    os::init_environment(env as *const *const i8); 

    sys_common::rt::init(argc as isize, argv);
    let result = main(argc as isize, argv); 
    sys_common::rt::cleanup();

    abi::exit(result); 
