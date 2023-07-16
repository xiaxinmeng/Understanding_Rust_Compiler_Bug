
#![feature(start, std_misc)]

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    unsafe { ::std::rt::args::init(argc, argv); }
    // ...                                                                                            
    return 0;
}
