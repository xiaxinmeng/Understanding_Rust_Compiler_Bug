 rust
type Context = Arc<RwLock<HashMap<Uuid, User>>>;

fn disconnect_user(ctx: Context, uid: Uuid) -> Result<String, String> {
    // ....
   match ctx.write().unwrap().remove(&uid) {
         Some(user) => { Ok(format!("user removed")) },
         None => { Err(format!("user not removed")) },
    }
}
