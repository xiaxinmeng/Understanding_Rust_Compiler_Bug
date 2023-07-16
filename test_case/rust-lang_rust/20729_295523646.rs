rust
error[E0194]: type parameter `T` shadows another type parameter of the same name
  --> <anon>:18:27
   |
15 | trait Bar<T> {
   |           - first `T` declared here
...
18 |     fn shadow_in_required<T>(&self);
   |                           ^ shadows another type parameter
