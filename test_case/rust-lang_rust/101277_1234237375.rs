rust
let mut x = 1;
{
     ...
     drop_mutability!(x);
}
x = 2;
