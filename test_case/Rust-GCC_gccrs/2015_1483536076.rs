
test.rs:11:25: note: lowering type!!!: MacroInvocation: [regular] 
 outer attributes: none                                                               
 foo!(())                                                                             
 has semicolon: false                                                                 
   11 | pub fn foo<T: Bar<Baz = foo!()>>(_value: T) -> i32 {     
      |                         ^~~                                                   
crab1: internal compiler error: in translate, at rust/hir/rust-ast-lower-type.h:73
