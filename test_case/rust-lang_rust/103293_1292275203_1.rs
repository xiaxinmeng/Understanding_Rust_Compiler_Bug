
[INFO] [stdout] error[E0716]: temporary value dropped while borrowed
[INFO] [stdout]    --> src/entangle/transform.rs:459:51
[INFO] [stdout]     |
[INFO] [stdout] 459 |         ReturnType::Type(_, ty) if !(is_name(ty, &name.to_string()) || is_name(ty, "Self"))
[INFO] [stdout]     |                                                   ^^^^^^^^^^^^^^^^-    ------- borrow later used here
[INFO] [stdout]     |                                                   |               |
[INFO] [stdout]     |                                                   |               temporary value is freed at the end of this statement
[INFO] [stdout]     |                                                   creates a temporary which is freed while still in use
