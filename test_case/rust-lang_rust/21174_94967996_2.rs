 shell
main.rs:7:30: 7:49 error: cannot transmute to or from a type that contains type parameters in its interior [E0139]
main.rs:7     let new: T::B = unsafe { std::mem::transmute(value) };
