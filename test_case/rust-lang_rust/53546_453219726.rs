
error[E0446]: private type `[closure@noria/src/table.rs:295:31: 295:44]` in public interface
   --> noria/src/table.rs:219:5
    |
219 |     existential type Future: Future<Item = Tagged<()>, Error = TableError>;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
...
295 |                     .fold((), |_, _| Ok(()))
    |                               ------------- `[closure@noria/src/table.rs:295:31: 295:44]` declared as private
