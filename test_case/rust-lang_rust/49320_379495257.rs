
failures:

---- [ui] ui/allocator-submodule.rs stdout ----
	normalized stderr:
error[E0432]: unresolved import `MY_HEAP`
  --> $DIR/allocator-submodule.rs:22:5
   |
LL |     static MY_HEAP: Heap = Heap::default();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MY_HEAP` in the root

error: aborting due to previous error
