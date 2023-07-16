
LL | use foo::{A, A};
   |           -
   |           |---
   |           || ^
   |           || |
   |           || `A` reimported here
   |           |help: remove unnecessary import
   |           previous import of the type `A` here
