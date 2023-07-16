
test.rs:6:50: 6:59 error: the value of the associated type `ChildKey` (from the trait `Hierarchy`) must be specified [E0191]
test.rs:6     type Children = Index<Self::ChildKey, Output=Hierarchy>;
                                                           ^~~~~~~~~
test.rs:6:50: 6:59 error: the value of the associated type `Children` (from the trait `Hierarchy`) must be specified [E0191]
test.rs:6     type Children = Index<Self::ChildKey, Output=Hierarchy>;
                                                           ^~~~~~~~~
test.rs:6:50: 6:59 error: the value of the associated type `Value` (from the trait `Hierarchy`) must be specified [E0191]
test.rs:6     type Children = Index<Self::ChildKey, Output=Hierarchy>;
                                                           ^~~~~~~~~
error: aborting due to 3 previous errors
