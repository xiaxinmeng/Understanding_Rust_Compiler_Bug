rust
fn count_cats(con: DBConnection, cuteness: u64) -> u64 {
    // sql! is able to read from data parsed in from new!
    // eg, could type check the database query
    // if a field in db is a set, then it can be added to, but not read from
    // otherwise it can be read from, but not written to
    db.sql!(con, select count(*) from cats where cuteness >= $cuteness)
}
