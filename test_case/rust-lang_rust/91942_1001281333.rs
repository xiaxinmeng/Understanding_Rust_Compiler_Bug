diff
+ fn funnel<'lt> (f: impl Fn(&mut State<'lt, '_>))
+   -> impl Fn(&mut State<'lt, '_>)
+ where
+     'static : 'lt, // make it turbofishable
+ {
+     f
+ }

  impl<'db> Trait<'db> for Impl<'db> {
      fn foo(&self, s: &mut State<'db, '_>) {
-         Car::new(s).debug(s, &mut (|s| {
+         Car::new(s).debug(s, &mut funnel::<'db>(|s| {
              self.some_trait.foo(s);
          }));
      }
  }
