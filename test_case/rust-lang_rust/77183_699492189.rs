
-       error: aborting due to 5 previous errors
+       error[E0658]: use of unstable library feature 'struct2_field'
+         --> $DIR/internal-unstable.rs:31:35
+          |
+       LL |     |x: internal_unstable::Bar| { access_field_allow2!(x) }; // regression test for #77088
+          |                                   ^^^^^^^^^^^^^^^^^^^^^^^
+          |
+          = help: add `#![feature(struct2_field)]` to the crate attributes to enable
+          = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+
+       error: aborting due to 6 previous errors
