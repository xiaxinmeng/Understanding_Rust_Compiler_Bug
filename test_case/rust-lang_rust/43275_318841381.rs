

  | fn foo(mut x: Vec<Ref<T>>,
  |                  --- 
  |       y: Ref<T>) 
  |          --- these two structs are not declared with the same lifetime ...
  |{
  | x.push(y);
  |        ^ ... but data flows from `y` into `x` here
  | }
help: consider changing the signature to:
  | fn foo<'a>(mut x: Vec<'a, Ref<T>>, y: Ref<'a, T>)
