text
error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:38 ~ aaaaaaaa_servo_arc[2007]::{impl#0}::new), const_param_did: None }) (after phase change to runtime-optimized) at bb2[1]:
                                Unable to compute layout for source type *mut T: the type `<T as Pointee>::Metadata` has an unknown layout
  --> C:\src\rust\tests\codegen\aaaaaaaa-servo-arc.rs:94:32
   |
94 |         NonZeroPtrMut(unsafe { mem::transmute(ptr) })
   |                                ^^^^^^^^^^^^^^^^^^^
   |
