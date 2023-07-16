rust
mod mod_migrations;

#[test]
fn demo() {
    assert_eq!(mod_migrations::migrations::V1__initial::migration(), "V1");
}
