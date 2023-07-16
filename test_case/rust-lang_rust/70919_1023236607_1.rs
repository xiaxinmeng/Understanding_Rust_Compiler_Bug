
error[E0506]: cannot assign to `self.conn` because it is borrowed
   --> mod_opcua/src/lib.rs:303:17
    |
296 |         let the_ref = self.conn.as_ref().unwrap();
    |                       ------------------ borrow of `self.conn` occurs here
297 |         let mut wg = match the_ref.write() {
    |                            --------------- a temporary with access to the borrow is created here ...
...
303 |                 self.conn = None;
    |                 ^^^^^^^^^ assignment to borrowed `self.conn` occurs here
...
306 |         };
    |          - ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Result<RwLockWriteGuard<'_, opcua_client::prelude::Session>, PoisonError<RwLockWriteGuard<'_, opcua_client::prelude::Session>>>`
