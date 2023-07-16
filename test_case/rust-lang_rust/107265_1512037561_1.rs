
error: unconstrained generic constant                                                                                                                                                        
    --> src/main.rs:1760:9                                                                                                                                                                   
     |                                                                                                                                                                                       
1760 |         [(); <NonNullPointer<T>>::SIZE]:,                                                                                                                                             
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                               
     |                                                                                                                                                                                       
     = help: try adding a `where` bound using this expression: `where [(); <NonNullPointer<T>>::SIZE]:`                                                                                      
note: required by a bound in `target::Copied<'a, target::Pointer<T>>`                                                                                                                        
    --> src/main.rs:1760:14                                                                                                                                                                  
     |                                                                                                                                                                                       
1760 |         [(); <NonNullPointer<T>>::SIZE]:,                                                                                                                                             
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Copied<'a, Pointer<T>>`                                                                                             
                                                                                                                                                                                             
error: unconstrained generic constant                                                                                                                                                        
    --> src/main.rs:1760:9                                                                                                                                                                   
     |                                                                                                                                                                                       
1760 |         [(); <NonNullPointer<T>>::SIZE]:,                                                                                                                                             
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^                                                                                                                                               
     |
     = help: try adding a `where` bound using this expression: `where [(); <Pointer<T>>::SIZE]:`
note: required by a bound in `target::Copied<'a, target::Pointer<T>>`
    --> src/main.rs:1761:14
     |
1761 |         [(); <Pointer<T>>::SIZE]:,
     |              ^^^^^^^^^^^^^^^^^^ required by this bound in `Copied<'a, Pointer<T>>`

error: unconstrained generic constant
    --> src/main.rs:1758:23
     |
1758 |     impl<'a, T: Type> Copied<'a, Pointer<T>>
     |                       ^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: try adding a `where` bound using this expression: `where [(); T::SIZE]:`
note: required by a bound in `target::Copied`
    --> src/main.rs:1731:14
     |
1729 |     pub enum Copied<'a, T: Type>
     |              ------ required by a bound in this enum
1730 |     where
1731 |         [(); T::SIZE]:,
     |              ^^^^^^^ required by this bound in `Copied`

