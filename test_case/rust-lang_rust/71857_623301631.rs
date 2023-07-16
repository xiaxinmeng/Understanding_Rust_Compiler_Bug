
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> $DIR/const_refers_to_static2.rs:14:18
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
   |                  ^^^
help: skipping check for `const_raw_ptr_deref` feature
  --> $DIR/const_refers_to_static2.rs:14:14
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> $DIR/const_refers_to_static2.rs:22:6
   |
LL |     &FOO
   |      ^^^
