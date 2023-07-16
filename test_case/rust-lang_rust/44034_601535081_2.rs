rust
fn main() {
    let con = postgresql::Connection::new('10.42.42.42');

    // all_queries is generated from a function that runs in a later stage
    // of the compilation, and gets linked in like normal.
    // It must have a fixed type signature (can change biased on params to new!)
    // The function is able to read from all of db's variables, but write to none
    con.prepare(db.all_queries!);
    // expanded to: con.prepare(magic_crate_db::all_queries);

    // use the preprocessed query
    println!("There are {} extra cute cats!", count_cats(con, 4242));
}
