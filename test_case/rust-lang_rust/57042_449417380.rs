
failures:

---- [ui] ui/huge-enum.rs stdout ----
diff of stderr:

-       error: the type `std::option::Option<[u32; N]>` is too big for the current architecture
+       error: the type `[u32; 35184372088831]` is too big for the current architecture
2
3       error: aborting due to previous error
4
