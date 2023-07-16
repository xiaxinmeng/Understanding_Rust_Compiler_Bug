
  | fn foo(mut x: Vec<Ref<T>>,
  |                  --- 
  |       y: Ref<T>) 
  |          --- these two structs are not declared with the same lifetime ...
  |{
  | x.push(y);
  |        ^ ... but data flows from `y` into `x` here
  | }
