
        _13 = &_6;                       // scope 2 at test_fails.rs:27:9: 27:11
        _12 = &(*(*_13));                // scope 2 at test_fails.rs:27:9: 27:11
        _11 = baz(move _12) -> [return: bb3, unwind: bb29]; // scope 2 at test_fails.rs:27:5: 27:12
