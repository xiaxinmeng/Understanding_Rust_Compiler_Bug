rust
let foo = <Something that causes a type error>; // We emit the original error that the user can fix
bar(foo); // We hide a type error that will be fixed if `foo` is fixed.
