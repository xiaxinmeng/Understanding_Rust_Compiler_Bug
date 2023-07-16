rust
use diesel::Connection;
use rocket_contrib::database;

#[database("test_db")]
pub struct TestDb(diesel::PgConnection);

use crate::rocket;
