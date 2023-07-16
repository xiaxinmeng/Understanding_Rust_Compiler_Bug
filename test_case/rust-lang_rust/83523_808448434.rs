rust
#[post("/", data = "<new_company>", format = "application/json")]
fn create_company(conn: DbConn, new_company: Json<NewCompany>) -> Json<Company> {
    let result = diesel::insert_into(companies::table)
        .values(&new_company.0)
        .get_result(&*conn);
    result.map(Json)
}
