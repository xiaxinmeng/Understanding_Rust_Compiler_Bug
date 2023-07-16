none
error[E0282]: type annotations needed for `&mut iter::adapters::ResultShunt<O, ()>`
   --> src/libcore/iter/traits/accum.rs:183:37
    |
183 |         OptionShunt::process(iter, |i| i.sum())
    |                                     ^ consider giving this closure parameter the explicit type `&mut iter::adapters::ResultShunt<O, ()>`, where the type parameter `O` is specified

error[E0277]: expected a `ops::function::FnMut<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` closure, found `F`
    --> src/libcore/iter/adapters/mod.rs:2081:9
     |
2081 |         ResultShunt::process(iter.map(|x| x.ok_or(())), f).ok()
     |         ^^^^^^^^^^^^^^^^^^^^ expected an `FnMut<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` closure, found `F`
     |
     = help: the trait `for<'r> ops::function::FnMut<(&'r mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` is not implemented for `F`
     = help: consider adding a `where for<'r> F: ops::function::FnMut<(&'r mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` bound
note: required by `iter::adapters::ResultShunt::<I, E>::process`
    --> src/libcore/iter/adapters/mod.rs:2102:5
     |
2102 | /     pub fn process<F, U>(iter: I, mut f: F) -> Result<U, E>
2103 | |         where F: FnMut(&mut Self) -> U
2104 | |     {
2105 | |         let mut shunt = ResultShunt::new(iter);
2106 | |         let value = f(shunt.by_ref());
2107 | |         shunt.reconstruct(value)
2108 | |     }
     | |_____^

error[E0277]: expected a `ops::function::FnOnce<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` closure, found `F`
    --> src/libcore/iter/adapters/mod.rs:2081:9
     |
2081 |         ResultShunt::process(iter.map(|x| x.ok_or(())), f).ok()
     |         ^^^^^^^^^^^^^^^^^^^^ expected an `FnOnce<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` closure, found `F`
     |
     = help: the trait `ops::function::FnOnce<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` is not implemented for `F`
     = help: consider adding a `where F: ops::function::FnOnce<(&mut iter::adapters::ResultShunt<iter::adapters::Map<I, [closure@src/libcore/iter/adapters/mod.rs:2081:39: 2081:54]>, ()>,)>` bound
note: required by `iter::adapters::ResultShunt::<I, E>::process`
    --> src/libcore/iter/adapters/mod.rs:2102:5
     |
2102 | /     pub fn process<F, U>(iter: I, mut f: F) -> Result<U, E>
2103 | |         where F: FnMut(&mut Self) -> U
2104 | |     {
2105 | |         let mut shunt = ResultShunt::new(iter);
2106 | |         let value = f(shunt.by_ref());
2107 | |         shunt.reconstruct(value)
2108 | |     }
     | |_____^
