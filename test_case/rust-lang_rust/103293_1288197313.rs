
[INFO] [stdout] error[E0716]: temporary value dropped while borrowed
[INFO] [stdout]    --> src/entangle/transform.rs:459:51
[INFO] [stdout]     |
[INFO] [stdout] 459 |         ReturnType::Type(_, ty) if !(is_name(ty, &name.to_string()) || is_name(ty, "Self"))
[INFO] [stdout]     |                                                   ^^^^^^^^^^^^^^^^-    ------- borrow later used here
[INFO] [stdout]     |                                                   |               |
[INFO] [stdout]     |                                                   |               temporary value is freed at the end of this statement
[INFO] [stdout]     |                                                   creates a temporary which is freed while still in use
[INFO] [stdout]     |
[INFO] [stdout] help: consider using a `let` binding to create a longer lived value
[INFO] [stdout]     |
[INFO] [stdout] 457 ~     let binding = name.to_string();
[INFO] [stdout] 458 ~     if matches!(
[INFO] [stdout] 459 |         &sig.output,
[INFO] [stdout] 460 ~         ReturnType::Type(_, ty) if !(is_name(ty, &binding) || is_name(ty, "Self"))
[INFO] [stdout]     |
