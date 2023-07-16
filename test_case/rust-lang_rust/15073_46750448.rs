
<anon>:16:5: 18:6 note: consider using an explicit lifetime parameter as shown: fn baz(&'a mut self)
<anon>:16     fn baz(&mut self) {
<anon>:17         let a = self.get();
<anon>:18     }
<anon>:17:17: 17:27 error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
<anon>:17         let a = self.get();
