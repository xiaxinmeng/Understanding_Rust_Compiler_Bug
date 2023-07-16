
// Today:
RC.set(...); // autoref of static "RC" lvalue because:

scoped_thread_local!(static RC: Rc<u8>);
// = expands to =>
static RC: ScopedKey<Rc<u8>> = ...;

// With this change:
RC.set(...); // autoref of local temporay "RC" rvalue because

scoped_thread_local!(static RC: Rc<u8>);
// = expands to =>
struct RC;
impl Deref<Target=Rc<u8>> for RC { 
    static mut ... // can have a !Sync in a static mut
    ... 
}
impl !Sync for RC {}
