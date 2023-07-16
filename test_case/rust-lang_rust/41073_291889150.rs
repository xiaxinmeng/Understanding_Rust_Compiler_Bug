rust
{
    let mut test = Test { a: A(3) }; // test.a is initialized -> test.b is initialized, test is initialized 
    {
        let tmp_b = test.b; // test.b is uninitialized (moved out) -> test.a is uninitialized, test is uninitialized
                            // tmp_b is initialized 
    } // tmp_b is dropped, B is printed
    test.b = B(0.5); // test.b is initialized -> test.a is initialized, test is initialized
                     // test.b is not dropped on assignment because it was uninitialized 
} // test is dropped (noop)
