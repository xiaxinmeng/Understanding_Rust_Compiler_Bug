rust
#[post("/task"), data = "<task>"] // Syntax error!
fn create_task(db: State<Arc<sled::Tree>>, task: Json<Task>) -> status::Accepted<String> {
    status::Accepted(Some(format!("success")))
}
