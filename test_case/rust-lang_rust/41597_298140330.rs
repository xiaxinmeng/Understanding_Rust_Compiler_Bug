
error[E0308]: mismatched types
  --> <anon>:37:21
   |
37 |         self.bridge(f)
   |                     ^ expected struct `MyFoo`, found type parameter
   |
   = note: expected type `MyFoo`
              found type `F`
