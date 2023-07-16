
+----------------------------+
| Bar vtable for some type T |
|----------------------------|
| Foo vtable                 | ----------> +------------------+
| foo_method                 | -----+      | Foo vtable for T |
| bar_method                 | --+  |      |------------------|
+----------------------------+   |  |      | foo_method       | ---+
                                 |  |      +------------------+    |
                                 |  |                              |
                                 |  +---> <T as Foo>::foo_method <-+
                                 v
                     <T as Bar>::bar_method
                 
(this diagram is only meant to convey the general idea of the vtable
 layout, and doesn't represent the exact offsets we would use etc;
 in fact, with the current implementation,
 the first supertrait is stored 'inline' and hence no load is required)
