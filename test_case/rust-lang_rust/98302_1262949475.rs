
$ nm target/debug/deps/thinlto_test-17415e9ded1c2189.thinlto_test-c418b5c87981f3e8.13mswjpwc6x43k1o.rcgu.o.rcgu.o | rg _ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E
0000000000000000 D _ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E

$ nm target/debug/deps/thinlto_test-17415e9ded1c2189.lg950o7fadi5c1i.rcgu.o | rg _ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E
0000000000000000 r .rdata$.refptr._ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E
0000000000000000 R .refptr._ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E
                 U _ZN12thinlto_test3aaa1F17ha0fefd1a54397fe9E
