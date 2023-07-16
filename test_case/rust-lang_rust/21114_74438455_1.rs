
src/supervisor/handlers/connections.rs:162:8: 162:11 error: `ctx` does not live long enough
src/supervisor/handlers/connections.rs:162      match ctx.users.write().ok().unwrap().remove(&uid) {
                                                      ^~~
src/supervisor/handlers/connections.rs:153:78: 186:2 note: reference must be valid for the destruction scope surrounding block at 153:77...
src/supervisor/handlers/connections.rs:153 pub fn disconnect_user(req: &Request, env: &mut Env) -> Result<String,String>{
src/supervisor/handlers/connections.rs:154      let ctx = env.get::<Arc<Context>>().unwrap().clone();
src/supervisor/handlers/connections.rs:155      let io  = env.get::<Sender<Envelope>>().unwrap().clone();
src/supervisor/handlers/connections.rs:156      let uid = req.uid.clone();
src/supervisor/handlers/connections.rs:157 
src/supervisor/handlers/connections.rs:158      let reason = req.payload.as_ref().and_then(|pl| {
                                           ...
src/supervisor/handlers/connections.rs:154:54: 186:2 note: ...but borrowed value is only valid for the block suffix following statement 0 at 154:53
src/supervisor/handlers/connections.rs:154      let ctx = env.get::<Arc<Context>>().unwrap().clone();
src/supervisor/handlers/connections.rs:155      let io  = env.get::<Sender<Envelope>>().unwrap().clone();
src/supervisor/handlers/connections.rs:156      let uid = req.uid.clone();
src/supervisor/handlers/connections.rs:157 
src/supervisor/handlers/connections.rs:158      let reason = req.payload.as_ref().and_then(|pl| {
src/supervisor/handlers/connections.rs:159              json::decode(&pl[]).ok()
                                           ...
error: aborting due to previous error
