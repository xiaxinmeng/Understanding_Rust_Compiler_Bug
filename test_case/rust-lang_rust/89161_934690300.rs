
error: internal compiler error: bad_placeholder_type
  --> ice.rs:11:64
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   |                                                                ^
   |
   = note: delayed at compiler/rustc_typeck/src/collect.rs:390:20
