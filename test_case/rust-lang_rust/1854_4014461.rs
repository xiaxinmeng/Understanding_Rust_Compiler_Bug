
// just declare a variable that happens to be a function...
let foo1 = ( x : int ) -> int {
   x+5
};

// alternative form, do inference the other way, either would be legal
let foo2 : (int) -> int = x {
  x+5
};

// full syntax, all types explicit, no inference
let foo3 : (int)->int = (x : int) -> int {
   x+5
};
