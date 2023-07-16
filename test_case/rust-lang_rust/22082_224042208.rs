
oo.rs:6:25: 6:35 error: cannot refer to other statics by value, use the address-of operator or a constant instead [E0394]
foo.rs:6                         TEST_CONST ... 17 => {},
                                 ^~~~~~~~~~
foo.rs:6:25: 6:35 error: constants cannot refer to other statics, insert an intermediate constant instead [E0013]
foo.rs:6                         TEST_CONST ... 17 => {},
                                 ^~~~~~~~~~
foo.rs:6:25: 6:35 error: non-constant path in constant expression
foo.rs:6                         TEST_CONST ... 17 => {},
                                 ^~~~~~~~~~
