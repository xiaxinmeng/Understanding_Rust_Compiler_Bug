
error[E0320]: overflow while adding drop-check rules for S<i32>
 --> <anon>:7:5
  |
7 |     S { a: None, b: 0i32 };
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
note: overflowed on struct S field a type: std::option::Option<Box<S<((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((i32,),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),)>>>
 --> <anon>:7:5
  |
7 |     S { a: None, b: 0i32 };
  |     ^^^^^^^^^^^^^^^^^^^^^^

error[E0320]: overflow while adding drop-check rules for std::option::Option<Box<S<(i32,)>>>
 --> <anon>:7:12
  |
7 |     S { a: None, b: 0i32 };
  |            ^^^^
  |
note: overflowed on enum std::option::Option variant Some field 0 type: Box<S<((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((i32,),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),),)>>
 --> <anon>:7:12
  |
7 |     S { a: None, b: 0i32 };
  |            ^^^^

error: aborting due to 2 previous errors
