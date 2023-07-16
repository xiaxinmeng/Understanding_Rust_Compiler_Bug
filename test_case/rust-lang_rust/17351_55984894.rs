
fn check_user(&self, username: &str) -> bool
{
    let conn = self.superconn.as_ref().unwrap();

    let stmt = conn.prepare(
        "SELECT 1 FROM pg_roles WHERE rolname=$1" )
        .unwrap_or_else(|e| {
            fail!("Could not check if database role {} exists (prepare): {}",username,e)
        });

    stmt.query(&[&username])
        .unwrap_or_else(|e| {
            fail!("Could not check if database role {} exists (query): {}",username,e)
        })
        .count() > 0;
}
