
no_obj_safety.rs:6:9: 6:14 error: the trait `Tr` is not implemented for the type `Tr` [E0277]
no_obj_safety.rs:6     obj.me(1);
                           ^~~~~
no_obj_safety.rs:17:15: 17:17 error: cannot convert to a trait object because trait `Tr` is not object-safe [E0038]
no_obj_safety.rs:17     let obj = &b as &Tr;
                                  ^~
no_obj_safety.rs:17:15: 17:17 note: method `me` has generic type parameters
no_obj_safety.rs:17     let obj = &b as &Tr;
                                  ^~
error: aborting due to 2 previous errors
