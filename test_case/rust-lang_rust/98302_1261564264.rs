
$ nm target/release/deps/thinlto_test-f0129d30483da959.thinlto_test.63a4c5c1-cgu.0.rcgu.o | rg thinlto_test3aaa1F17h679dc1481bd45021E
                 U __imp__ZN12thinlto_test3aaa1F17h679dc1481bd45021E

$ nm target/release/deps/thinlto_test-f0129d30483da959.thinlto_test-72d93203162d98ec.thinlto_test.a44eecf0-cgu.0.rcgu.o.rcgu.o | rg thinlto_test3aaa1F17h679dc1481bd45021E
0000000000000000 D _ZN12thinlto_test3aaa1F17h679dc1481bd45021E
