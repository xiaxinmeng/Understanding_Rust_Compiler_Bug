
   Compiling playground v0.0.1 (file:///playground)
error[E0392]: parameter `U` is never used
 --> src/main.rs:2:18
  |
2 | struct Cacher<T, U: Copy>
  |                  ^ no member type stores or points to this type
  |
  = help: check if any member should be storing or pointing to `U` (you can use `std::marker::PhantomData` to fake this)
