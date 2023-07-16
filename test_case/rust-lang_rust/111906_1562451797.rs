diff
fn foo<'a: 'a>() -> impl Sized {
-    let _: &'a () = foo::<'a>();
+    let _: *mut &'a () = foo::<'a>();
    loop {}
}
