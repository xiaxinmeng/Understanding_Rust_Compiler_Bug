
warning[E0716]: temporary value dropped while borrowed
  --> $DIR/dangling-alloc-id-ice-2.rs:5:28
   |
LL |    static MAP: Slice = Slice(&[
   |   ___________________________-^
   |  |___________________________|
   | ||
LL | ||     b"CloseEvent" as &'static [u8],
LL | || ]);
   | || -- temporary value is freed at the end of this statement
   | ||_|
   | |__creates a temporary which is freed while still in use
   |    cast requires that borrow lasts for `'static`
   |
   = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
           It represents potential unsoundness in your code.
           This warning will become a hard error in the future.
