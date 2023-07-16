
error[E0283]: type annotations needed
  --> /home/obei/rust/rust/src/test/ui/associated-consts/issue-63496.rs:4:21
   |
LL |     fn f() -> ([u8; A::C], [u8; A::C]);
   |                     ^^^^ cannot call trait method as a free function

error[E0283]: type annotations needed
  --> /home/obei/rust/rust/src/test/ui/associated-consts/issue-63496.rs:4:33
   |
LL |     fn f() -> ([u8; A::C], [u8; A::C]);
   |                                 ^^^^ cannot call trait method as a free function

